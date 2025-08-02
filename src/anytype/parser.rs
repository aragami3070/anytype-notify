use crate::{
    Token, Url,
    anytype::entities::api_response::{ApiResponse},
};

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

    let body = response.json::<ApiResponse>().await?;

    Ok(body)
}
