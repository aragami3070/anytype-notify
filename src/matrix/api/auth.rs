use crate::{Password, Token, User, matrix::client::Client};

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginRequest {
    identifier: Identifier,
    #[serde(rename = "initial_device_display_name")]
    device_name: Option<String>,
    password: String,
    #[serde(rename = "type")]
    login_type: String,
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    request_refresh_token: Option<bool>,
}

#[derive(Serialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    type_: String,
    user: String,
}

pub struct Auth {
    pub client: Client,
}

impl Auth {
    pub fn new(client: Client) -> Self {
        Auth { client }
    }
}
