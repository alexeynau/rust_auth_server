use actix_web::{
    get,
    http::{
        uri::{self, PathAndQuery},
        Uri,
    },
    post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Deserialize)]
struct UserResponse {
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

#[derive(Deserialize)]
struct AuthRequestBody {
    op: String,
    id: String,
    uuid: String,
    device_info: DeviceInfo,
}

#[derive(Deserialize)]
struct DeviceInfo {
    os: String,
    r#type: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct CodeUrl {
    code: String,
    url: String,
}
// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(user_response: web::Query<UserResponse>) -> String {
    let params = [
        ("client_id", dotenv::var("CLIENT_ID").unwrap()),
        ("client_secret", dotenv::var("CLIENT_SECRET").unwrap()),
        ("redirect_uri", String::from("http://localhost:8080")),
        ("grant_type", String::from("authorization_code")),
        ("code", user_response.code.clone()),
    ];
    let client = reqwest::Client::new();
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await;

    format!("Code: {}", res.unwrap().text().await.unwrap())
}

/// deserialize `Info` from request's body
#[post("/auth")]
async fn auth(auth_req_body: web::Json<AuthRequestBody>) -> Result<HttpResponse, Error> {
    dotenv().ok();
    // let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
    let _ = dotenv::from_filename(".env");
    let authority = "accounts.google.com";
    let path = "/o/oauth2/auth";
    let client_id = dotenv::var("CLIENT_ID").unwrap();
    let redirect_uri = "http://localhost:8080";
    let uri = Uri::builder()
        .scheme("https")
        .authority("accounts.google.com")
        .path_and_query(format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=profile%20email",
            path, client_id, redirect_uri
        ))
        .build()
        .unwrap();
    let code_url = CodeUrl {
        code: String::from("123"),
        url: uri.to_string(),
    };
    Ok(HttpResponse::Ok().json(code_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(auth).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
