use serde::{Deserialize, Serialize};
use std::{str::FromStr, collections::HashMap};




#[derive(Deserialize)]
pub struct UserResponse {
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub prompt: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthRequestBody {
    pub op: String,
    pub id: String,
    pub uuid: String,
    pub deviceInfo: DeviceInfo,
}

#[derive(Deserialize, Debug)]
pub struct AuthRequestQuery {
    pub code: String,
    pub id: String,
    pub uuid: String
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct DeviceInfo {
//     pub os: String,
//     pub r#type: String,
//     pub name: String,
// }

#[derive(Deserialize, Serialize)]
pub struct CodeUrl {
    pub code: String,
    pub url: String,
}

// #[derive(Deserialize, Serialize)]
// pub struct User {
//     pub name: String,
//     pub email: Option<String>,
//     pub note: Option<String>,
//     pub status: i8,
//     pub info: UserInfo,

// }
// #[derive(Deserialize, Serialize)]
// pub struct UserInfo {
//     pub settings: Setting,
//     pub login_device_whitelist: Vec<LoginDeviceWhiteListElement>,
//     pub other: Option<HashMap<String, String>>
// }
// #[derive(Deserialize, Serialize)]
// pub struct Setting {
//     pub email_verification: bool,
//     pub email_alarm_notification: bool,
// }
// #[derive(Deserialize, Serialize)]
// pub struct LoginDeviceWhiteListElement{
//     pub data: String,
//     pub info: DeviceInfo,
//     pub exp: i64
// }

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

pub struct AuthQuery {
    pub code: String,
    pub id: String,
    pub uuid: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct AuthQueryResponse {
//     pub access_token: String,
//     pub r#type: String,
//     pub user: User,
//     pub is_admin: bool,
//     pub third_auth_type: String
// }



#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct DeviceInfo {
    /// Linux , Windows , Android ...
    #[serde(default)]
    pub os: String,

    /// `browser` or `client`
    #[serde(default)]
    pub r#type: String,

    /// device name from rustdesk client,
    /// browser info(name + version) from browser
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WhitelistItem {
    data: String, // ip / device uuid
    info: DeviceInfo,
    exp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserInfo {
    #[serde(default, flatten)]
    pub settings: UserSettings,
    #[serde(default)]
    pub login_device_whitelist: Vec<WhitelistItem>,
    #[serde(default)]
    pub other: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserSettings {
    #[serde(default)]
    pub email_verification: bool,
    #[serde(default)]
    pub email_alarm_notification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPayload {
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub status: i64,
    pub info: UserInfo,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub third_auth_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBody {
    pub access_token: String,
    pub r#type: String,
    pub user: UserPayload,
}