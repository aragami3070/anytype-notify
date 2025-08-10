use crate::{
    Token, Url,
    anytype::entities::{api_response::ApiResponse, notification::AnytypeToMatrixIdMap},
};

use reqwest::{Client, header::HeaderMap};
use std::{collections::HashMap, error::Error};

/// Get all Anytype objects from space
pub async fn get_anytype_objects(
    anytype_url: &Url,
    anytype_token: &Token,
) -> Result<ApiResponse, Box<dyn Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse()?);
    headers.insert(
        "Authorization",
        format!("Bearer {}", anytype_token.0).parse()?,
    ); // Anytype API token

    let response = client
        .get(anytype_url.0.clone())
        .headers(headers)
        .send()
        .await?;

    // Check if the request was unsuccessful
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Error: bad status from Anytype API: {status}. Body: {body}").into());
    }

    // Decode the response to ApiResponse structure
    let text = response.text().await?;
    let body: ApiResponse = serde_json::from_str(&text)
        .map_err(|e| format!("Error: decoding response body: {e}. Raw response: {text}"))?;

    Ok(body)
}

/// Get a mapping for finding the matrix id of the user by the anytype space member id
pub async fn get_anytype_to_matrix_map(
    anytype_url: &Url,
    anytype_token: &Token,
    map_type: &str, // The name of the Anytype object type which contains the "anytype_id" and "matrix_id" properties
) -> Result<AnytypeToMatrixIdMap, Box<dyn Error>> {
    let mut map = HashMap::new();
    let all_objects = get_anytype_objects(anytype_url, anytype_token).await?;

    for o in &all_objects.data {
        // Skip objects that are not of the specified type
        if o.type_field.as_ref().map(|t| t.key.as_str()) != Some(map_type) {
            continue;
        }

        // Find the "anytype_id" and "matrix_id" properties
        let anytype_id = o
            .properties
            .iter()
            .find(|p| p.key == "anytype_id")
            .and_then(|p| p.objects.as_ref())
            .and_then(|obj| obj.first())
            .cloned();

        let matrix_id = o
            .properties
            .iter()
            .find(|p| p.key == "matrix_id")
            .and_then(|p| p.text.as_ref())
            .cloned();

        // If both properties are found, add them to the map
        if let (Some(anytype_id), Some(matrix_id)) = (anytype_id, matrix_id) {
            map.insert(anytype_id, matrix_id);
        }
    }

    Ok(AnytypeToMatrixIdMap { map })
}

/// Find the matrix id of the user by the anytype space member id in the map
pub fn find_matrix_user_id(map: &AnytypeToMatrixIdMap, anytype_id: &str) -> String {
    map.map
        .get(anytype_id)
        .cloned()
        .unwrap_or_else(|| "Unknown User".to_string())
}
