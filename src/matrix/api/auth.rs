use crate::{
    Token,
    matrix::client::{Client, Password, User},
};

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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WhoAmI {
    pub device_id: DeviceId,
    pub user_id: UserId,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct NewTokens {
    access_token: Token,
    expires_in_ms: i64,
    refresh_token: Token,
}

#[derive(Serialize)]
pub struct RefreshRequest {
    pub refresh_token: Token,
}

pub struct Auth {
    pub client: Client,
}

impl Auth {
    pub fn new(client: Client) -> Self {
        Auth { client }
    }

    /// Эта функция делает **post** запрос к **/_matrix/client/v3/login** для входа в аккаунт по имени
    /// и паролю пользователя
    ///
    /// Добавляет в ```Client``` полученные токены и возвращает ```Client```
    pub async fn login(mut self, user: User, password: Password) -> Result<Client, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let identifier_val = Identifier {
            type_: "m.id.user".to_string(),
            user: user.0,
        };

        let body = LoginRequest {
            identifier: identifier_val,
            device_name: Some("anytype-bot docker".to_string()),
            password: password.0,
            login_type: "m.login.password".to_string(),
            request_refresh_token: Some(true),
        };

        let response = self
            .client
            .post("/_matrix/client/v3/login", headers, body)
            .await?;

        let result = match response.json::<LoginResponse>().await {
            Ok(resp) => resp,
            Err(message) => return Err(Box::new(message)),
        };

        self.client
            .set_tokens(result.access_token.clone(), result.refresh_token.clone());

        Ok(self.client)
    }

    /// Эта функция делает **get** запрос к **/_matrix/client/v3/account/whoami** для получения данных о
    /// данном сеансе. Если токен истек вернет ошибку
    pub async fn who_am_i(&self) -> Result<WhoAmI, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        headers.insert(
            "Authorization",
            format!("Bearer {}", self.client.get_access_token().0).parse()?,
        );

        let response = self
            .client
            .get("/_matrix/client/v3/account/whoami", headers)
            .await?;

        let result = match response.json::<WhoAmI>().await {
            Ok(resp) => resp,
            Err(message) => return Err(Box::new(message)),
        };

        Ok(result)
    }

    /// Эта функция делает **post** запрос к **/_matrix/client/v3/refresh** для обновления токенов.
    ///
    /// Добавляет в ```Client``` полученные токены и возвращает ```Client```
    pub async fn refresh(mut self) -> Result<Client, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let response = self
            .client
            .post(
                "/_matrix/client/v3/refresh",
                headers,
                RefreshRequest {
                    refresh_token: self.client.get_refresh_token(),
                },
            )
            .await?;

        let result = match response.json::<NewTokens>().await {
            Ok(resp) => resp,
            Err(message) => return Err(Box::new(message)),
        };

        self.client
            .set_tokens(result.access_token.clone(), result.refresh_token.clone());

        Ok(self.client)
    }
}
