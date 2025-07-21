use crate::{Password, Token, User, matrix::client::Client};

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceId(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserId(pub String);

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub access_token: Token,
    pub device_id: DeviceId,
    pub user_id: UserId,
    pub refresh_token: Token,
    pub expires_in_ms: i64,
}

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

    pub async fn login(
        &self,
        user: User,
        password: Password,
    ) -> Result<LoginResponse, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            match "application/json".parse() {
                Ok(s) => s,
                Err(message) => return Err(Box::new(message)),
            },
        );

        let identifier = Identifier {
            type_: "m.id.user".to_string(),
            user: user.0,
        };
        let body = LoginRequest {
            identifier: identifier,
            device_name: Some("anytype-bot docker".to_string()),
            password: password.0,
            login_type: "m.login.password".to_string(),
            request_refresh_token: Some(true),
        };

        let response = match self
            .client
            .post("/_matrix/client/v3/login", headers, body)
            .await
        {
            Ok(resp) => resp,
            Err(message) => return Err(message),
        };

        let result = match response.json::<LoginResponse>().await {
            Ok(resp) => resp,
            Err(message) => return Err(Box::new(message)),
        };

        Ok(result)
    }
}
