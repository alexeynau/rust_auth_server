use std::{collections::HashMap, str::FromStr};

use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use rust_auth_server::data::models::*;

// pub mod models;
// use crate::models::AuthRequestBody;
use serde_json;
// use crate::models::{self, CodeUrl, LoginOption};
/// this handler gets request with code in query params
pub async fn google_authorization(Query(user_response): Query<UserResponse>) -> String {
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
    println!("{}", text_response.as_str());
    let google_token_response: GoogleTokenResponse =
        serde_json::from_str(text_response.as_str()).unwrap();

    // let access_token = google_token_response.access_token.as_str();

    // verifying (decoding) JWT token
    let mut client = google_signin::Client::new();
    client.audiences.push(dotenv::var("CLIENT_ID").unwrap());
    let id_info = client
        .verify(&google_token_response.id_token)
        .expect("Expected token to be valid");
    println!("Success! Signed-in as {}", id_info.sub);

    format!(
        "OK\nYour login: {}\nYour name: {} {}",
        id_info.email.unwrap(),
        id_info.given_name.unwrap(),
        id_info.family_name.unwrap()
    )
}

pub async fn auth_query(
    Query(auth_req_query): Query<AuthRequestQuery>,
) -> Result<Json<AuthBody>, StatusCode> {
    println!("request body : {:?}", auth_req_query);
    let other = HashMap::<String, String>::new();
    Ok(Json(AuthBody {
        access_token: String::from("ya29.a0AfB_byDmNGQ-1L3mb-WMzTCTzG-DyFjenJuQtOnLeI0GRJ1NLxX2lpQ7yVbubngZxB9h_awWNjbFV6J-VNReHOus5HrE2DX9v1NV5j9Kzp3ZhVlQapTX82JWSR69d1xVyPwpXqi6pATVbhKBHVJKC7sHzer37h2mw55kaCgYKAWYSARASFQGOcNnCEiDeHaCsIv7MgYP7hJp0OA0171"),
        r#type: String::from("client"),
        user: UserPayload {
            name: String::from("Alexey"),
            email: Some(String::from("gmail@gmail.com")),
            status: 1,
            info: UserInfo {
                settings: UserSettings {
                    email_verification: true,
                    email_alarm_notification: true,
                },
                login_device_whitelist: Vec::<WhitelistItem>::from([]),
                other: other,
            },
            note: Some("important note".to_string()),
            is_admin: true,
            third_auth_type: Some("google".to_string()),
        },
    }))
}

pub async fn auth(Json(auth_req_body): Json<AuthRequestBody>) -> Result<Json<CodeUrl>, StatusCode> {
    // DB.insert(
    //     auth_req_body.id,
    //     AuthQueryResponse {
    //         access_token: String::from(""),
    //         r#type: auth_req_body.device_info.r#type,
    //         user: User {
    //             name: String::from(""),
    //             email: Some(String::from("")),
    //             status: 1,
    //             info: UserInfo {
    //                 settings: Setting {
    //                     email_verification: true,
    //                     email_alarm_notification: true,
    //                 },
    //                 login_device_whitelist: Vec::<LoginDeviceWhiteListElement>::from([]),
    //                 other: None,
    //             },
    //             note: None,
    //         },
    //         is_admin: false,
    //         third_auth_type: auth_req_body.op,
    //     },
    // );
    println!("request body : {:?}", auth_req_body);
    // get current option
    let current_option = LoginOption::from_str(auth_req_body.op.as_str());
    // handle the current option
    match current_option {
        Ok(LoginOption::Google) => {
            let code_url = CodeUrl {
                code: String::from("123"),
                url: get_google_auth_url(),
            };
            Ok(Json(code_url))
        }
        Ok(LoginOption::Telegram) => Err(StatusCode::NOT_IMPLEMENTED),
        Err(()) => Err(StatusCode::BAD_REQUEST),
    }
}

/// create a url for google auth
fn get_google_auth_url() -> String {
    let authority = "accounts.google.com";
    let path = "/o/oauth2/auth";
    let client_id = dotenv::var("CLIENT_ID").unwrap();
    let redirect_uri = "http://localhost:8080";
    let uri = format!(
        "https://{}{}?client_id={}&redirect_uri={}&response_type=code&scope=profile%20email",
        authority, path, client_id, redirect_uri
    );
    uri
}
