use std::str::FromStr;
use actix_web::{get, http::Uri, post, web, Error, HttpResponse};
use dotenv::dotenv;
use reqwest;
pub mod models;
use models::*;
use serde_json;
use google_signin;

/// this handler gets request with code in query params
#[get("/")]
async fn index(user_response: web::Query<UserResponse>) -> String {
    // set params for request for google
    let params = [
        ("client_id", dotenv::var("CLIENT_ID").unwrap()),
        ("client_secret", dotenv::var("CLIENT_SECRET").unwrap()),
        ("redirect_uri", String::from("http://localhost:8080")),
        ("grant_type", String::from("authorization_code")),
        ("code", user_response.code.clone()),
    ];

    let client = reqwest::Client::new();
    // exchange code for access token and user_info
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await;
    // deserialize response
    let text_response = res.unwrap().text().await.unwrap();
    println!("{}",text_response.as_str());
    let google_token_response: GoogleTokenResponse =
        serde_json::from_str(text_response.as_str()).unwrap();

    // let access_token = google_token_response.access_token.as_str();

    // verifying (decoding) JWT token 
    let mut client = google_signin::Client::new();
    client.audiences.push(dotenv::var("CLIENT_ID").unwrap()); 
    let id_info = client.verify(&google_token_response.id_token).expect("Expected token to be valid");
    println!("Success! Signed-in as {}", id_info.sub);

    format!(
        "OK\nYour login: {}\nYour name: {} {}", id_info.email.unwrap(), id_info.given_name.unwrap(), id_info.family_name.unwrap()
    )

}

/// take device info and login option in params and return code and url for authenfication
#[post("/auth")]
async fn auth(auth_req_body: web::Json<AuthRequestBody>) -> Result<HttpResponse, Error> {
    // get current option
    let current_option = LoginOption::from_str(auth_req_body.op.as_str());
    // handle the current option
    match current_option {
        Ok(LoginOption::Google) => {
            let code_url = CodeUrl {
                code: String::from("123"),
                url: get_google_auth_url(),
            };
            Ok(HttpResponse::Ok().json(code_url))
        }
        Ok(LoginOption::Telegram) => Err(actix_web::error::ErrorNotImplemented(
            "Telegram login option not implemented yet",
        )),
        Err(()) => Err(actix_web::error::ErrorBadRequest("Wrong login option")),
    }
}

/// create a url for google auth
fn get_google_auth_url() -> String{
    let authority = "accounts.google.com";
    let path = "/o/oauth2/auth";
    let client_id = dotenv::var("CLIENT_ID").unwrap();
    let redirect_uri = "http://localhost:8080";
    let uri = Uri::builder()
        .scheme("https")
        .authority(authority)
        .path_and_query(format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=profile%20email",
            path, client_id, redirect_uri
        ))
        .build()
        .unwrap();
    uri.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    dotenv().ok();
    let _ = dotenv::from_filename(".env");

    HttpServer::new(|| App::new().service(auth).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
