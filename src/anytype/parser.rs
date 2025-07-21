use crate::anytype::entities::api_response::{AnytypeObject, ApiResponse};
use crate::anytype::entities::api_response;

use reqwest::Client;
use reqwest::header::HeaderMap;
use std::error::Error;

async fn fetch(url: &String, token: &String) -> Result<ApiResponse, Box<dyn Error>> {
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

async fn filter_objects_by_types(
    objects: ApiResponse,
    required_types: &Vec<String>,
) -> Result<Vec<AnytypeObject>, String> {
    let filtered_objects: Vec<AnytypeObject> = objects
        .data
        .into_iter()
        .filter(|o| {
            o.type_field
                .as_ref()
                .map(|t| required_types.iter().any(|ty| ty == &t.key))
                .unwrap_or(false)
        })
        .collect();

    if filtered_objects.is_empty() {
        return Err(format!(
            "No objects found with required types: {:?}",
            required_types
        ));
    }

    Ok(filtered_objects)
}

pub async fn get_anytype_objects(
    url: &String,
    token: &String,
    required_types: &Vec<String>,
) -> Result<ApiResponse, Box<dyn Error>> {
    let objects = match fetch(url, token).await {
        Ok(data) => {
            println!("Response parsed successfully");
            data
        }
        Err(message) => {
            return Err(message.into());
        }
    };

    let filtred_objects = match filter_objects_by_types(objects, required_types).await {
        Ok(objects) => {
            println!("Found {} objects", objects.len());
            objects
        }
        Err(message) => {
            return Err(message.into());
        }
    };

    Ok(api_response::ApiResponse {
        data: filtred_objects,
    })
}
