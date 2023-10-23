
use serde::{Deserialize, Serialize};
use std::{str::FromStr, collections::HashMap};


#[derive(Deserialize)]
pub struct UserResponse {
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct AuthRequestBody {
    pub op: String,
    pub id: String,
    pub uuid: String,
    pub device_info: DeviceInfo,
}

#[derive(Deserialize, Serialize)]
pub struct DeviceInfo {
    pub os: String,
    pub r#type: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CodeUrl {
    pub code: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    name: String,
    email: String,
    note: String,
    status: i8,
    info: UserInfo,
    other: Option<HashMap<String, String>>
}
#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    settings: Setting,
    login_device_whitelist: Vec<LoginDeviceWhiteListElement>
}
#[derive(Deserialize, Serialize)]
pub struct Setting {
    email_verification: bool,
    email_alarm_notification: bool,
}
#[derive(Deserialize, Serialize)]
pub struct LoginDeviceWhiteListElement{
    data: String,
    info: DeviceInfo,
    exp: i64
}

#[derive(Debug, PartialEq)]
pub enum LoginOption {
    Google,
    Telegram,
}
impl FromStr for LoginOption {
    type Err = ();
     fn from_str(input: &str) -> Result<LoginOption, Self::Err> {
        match input {
            "google"  => Ok(LoginOption::Google),
            "telegram"  => Ok(LoginOption::Telegram),
            _  => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdTokenDecoded {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub at_hash: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
    pub iat: i64,
    pub exp: i64,
    pub header: Header,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}
