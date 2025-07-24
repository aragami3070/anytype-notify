use crate::{
    Token, Url,
    anytype::entities::api_response::{AnytypeObject, ApiResponse},
    anytype::parser::get_anytype_objects,
    config::AppConfig,
};

use std::{
    collections::HashSet,
    error::Error,
    fs::{self, File}, path::Path,
};

async fn save_to_cache(path: &str, objects: &ApiResponse) -> std::io::Result<()> {
    let cache_path = Path::new(path);

    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, objects)?;
    Ok(())
}

async fn load_from_cache(path: &str) -> std::io::Result<ApiResponse> {
    let data = fs::read_to_string(path)?;
    let objects: ApiResponse = serde_json::from_str(&data)?;
    Ok(objects)
}

async fn compare_with_cache(cached: &ApiResponse, current: &ApiResponse) -> ApiResponse {
    let cached_ids: HashSet<&str> = cached.data.iter().map(|o| o.id.as_str()).collect();
    let new_objects: Vec<AnytypeObject> = current
        .data
        .iter()
        .filter(|o| !cached_ids.contains(o.id.as_str()))
        .cloned()
        .collect();

    ApiResponse { data: new_objects }
}

/// Find Anytype objects with creation date after last check
pub async fn find_new_objects(anytype_url: &Url) -> Result<ApiResponse, Box<dyn Error>> {
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));

    let config = AppConfig::from_file("config.toml")?;
    let cache_path = "assets/cache.json";

    let current_objects =
        get_anytype_objects(anytype_url, &anytype_token, &config.required_types).await?;

    if !Path::new(cache_path).exists() {
        println!("Cache not found. Saving current objects and exiting.");
        save_to_cache(cache_path, &current_objects).await?;
        return Ok(ApiResponse { data: vec![] })
    }
    let cached_objects = load_from_cache(cache_path).await?;

    let new_objects = compare_with_cache(&cached_objects, &current_objects).await;

    save_to_cache(cache_path, &current_objects).await?;

    Ok(new_objects)
}
