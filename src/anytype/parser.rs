use crate::{
    RequiredTypes, Token, Url,
    anytype::entities::api_response::{self, AnytypeObject, ApiResponse},
};

use reqwest::Client;
use reqwest::header::HeaderMap;
use std::error::Error;

/// Get all Anytype objects from space 
async fn fetch(url: &Url, token: &Token) -> Result<ApiResponse, Box<dyn Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse()?);
    headers.insert("Authorization", format!("Bearer {}", token.0).parse()?);

    let response = client.get(url.0.clone()).headers(headers).send().await?;

    let body = response.json::<ApiResponse>().await?;

    Ok(body)
}

/// Find objects with only required types from Anytype response
async fn filter_objects_by_types(
    objects: ApiResponse,
    required_types: &RequiredTypes,
) -> Result<Vec<AnytypeObject>, String> {
    let filtered_objects: Vec<AnytypeObject> = objects
        .data
        .into_iter()
        .filter(|o| {
            o.type_field
                .as_ref()
                .map(|t| required_types.types.iter().any(|ty| ty == &t.key))
                .unwrap_or(false)
        })
        .collect();

    if filtered_objects.is_empty() {
        return Err(format!(
            "No objects found with required types: {required_types:?}"
        ));
    }

    Ok(filtered_objects)
}

/// Get Anytype objects with required types from space
pub async fn get_anytype_objects(
    url: &Url,
    token: &Token,
    required_types: &RequiredTypes,
) -> Result<ApiResponse, Box<dyn Error>> {
    let objects = fetch(url, token).await?;

    let filtred_objects = filter_objects_by_types(objects, required_types).await?;

    Ok(api_response::ApiResponse {
        data: filtred_objects,
    })
}
