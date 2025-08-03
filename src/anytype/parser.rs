use crate::{Token, Url, anytype::entities::api_response::ApiResponse};

use reqwest::Client;
use reqwest::header::HeaderMap;
use std::error::Error;

/// Get all Anytype objects from space
pub async fn get_anytype_objects(url: &Url, token: &Token) -> Result<ApiResponse, Box<dyn Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse()?);
    headers.insert("Authorization", format!("Bearer {}", token.0).parse()?);

    let response = client.get(url.0.clone()).headers(headers).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Error: bad status from Anytype API: {status}. Body: {body}").into());
    }

    let text = response.text().await?;
    let body: ApiResponse = serde_json::from_str(&text)
        .map_err(|e| format!("Error: decoding response body: {e}. Raw response: {text}"))?;

    Ok(body)
}
