use chrono::{Duration, Utc};

use crate::{
    Token, Url,
    anytype::entities::api_response::{self, AnytypeObject, ApiResponse},
    anytype::parser::get_anytype_objects,
    config::AppConfig,
};

use std::{error::Error};

/// Find Anytype objects with creation date after last check
pub async fn find_new_objects(anytype_url: &Url) -> Result<ApiResponse, Box<dyn Error>> {
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));

    let config = AppConfig::from_file("config.toml")?;

    let objects = get_anytype_objects(anytype_url, &anytype_token, &config.required_types).await?;

    let last_check_time = Utc::now() - Duration::minutes(config.interval_minutes);

    let new_objects: Vec<AnytypeObject> = objects.data.iter().filter(|o| {
        o.created_at()
            .map(|dt| dt >= last_check_time)
            .unwrap_or(false)
    }).cloned().collect();
    
    Ok(api_response::ApiResponse {
        data: new_objects,
    })
}
