use crate::anytype::entities::api_response::ApiResponse;

use reqwest::Client;
use reqwest::header::HeaderMap;
use std::error::Error;

pub async fn fetch(url: &String, token: &String) -> Result<ApiResponse, Box<dyn Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse()?);
    headers.insert("Authorization", format!("Bearer {token}").parse()?);

    let response = match client.get(url).headers(headers).send().await {
        Ok(r) => r,
        Err(message) => {
            return Err(Box::new(message));
        }
    };

    let body = match response.json::<ApiResponse>().await {
        Ok(b) => b,
        Err(message) => {
            return Err(Box::new(message));
        }
    };

    Ok(body)
}
