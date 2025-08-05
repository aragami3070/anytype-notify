use crate::{
    Token, Url,
    anytype::entities::api_response::{AnytypeObject, ApiResponse},
    anytype::entities::cache::{AnytypeCache, CachedObject},
    anytype::parser::get_anytype_objects,
};

use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

async fn save_to_cache(path: &str, objects: &AnytypeCache) -> std::io::Result<()> {
    let cache_path = Path::new(path);

    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, objects)?;
    Ok(())
}

async fn load_from_cache(path: &str) -> Result<AnytypeCache, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let cache: AnytypeCache = serde_json::from_str(&data)?;
    Ok(cache)
}

/// Find Anytype objects with creation date after last check
pub async fn find_new_objects(anytype_url: &Url) -> Result<ApiResponse, Box<dyn Error>> {
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));

    let cache_path = "assets/cache.json";

    let current_objects = get_anytype_objects(anytype_url, &anytype_token).await?;

    if !Path::new(cache_path).exists() {
        println!("Cache not found. Saving current objects and exiting.");
        let mut initial_cache = AnytypeCache::default();

        for o in &current_objects.data {
            let id = &o.id;
            let notify_flag = o.is_notify_enabled();
            let assignee = o.assignee();
            initial_cache.objects.insert(
                id.clone(),
                CachedObject {
                    notify: notify_flag,
                    notified: notify_flag,
                    assignee: assignee,
                },
            );
        }

        if let Err(e) = save_to_cache(cache_path, &initial_cache).await {
            eprintln!("Failed to save initial cache: {e}");
        }

        return Ok(ApiResponse { data: vec![] });
    }

    let mut cached_objects = load_from_cache(cache_path).await?;

    let mut objects_to_notify: Vec<AnytypeObject> = Vec::new();

    for o in &current_objects.data {
        let id = &o.id;
        let notify_flag = o.is_notify_enabled();
        let assignee = o.assignee();

        match cached_objects.objects.get_mut(id) {
            Some(obj) => {
                // Object exists in cache
                if notify_flag && !obj.notified {
                    objects_to_notify.push(o.clone());
                    obj.notify = true;
                    obj.notified = true;
                    obj.assignee = assignee;
                } else if !notify_flag {
                    obj.notify = false;
                    obj.notified = false;
                    obj.assignee = assignee;
                }
            }
            None => {
                // Object doesn't exist in cache
                if notify_flag {
                    objects_to_notify.push(o.clone());
                    cached_objects.objects.insert(
                        id.clone(),
                        CachedObject {
                            notify: true,
                            notified: true,
                            assignee: assignee,
                        },
                    );
                } else {
                    cached_objects.objects.insert(
                        id.clone(),
                        CachedObject {
                            notify: false,
                            notified: false,
                            assignee: assignee,
                        },
                    );
                }
            }
        }
    }

    if let Err(e) = save_to_cache(cache_path, &cached_objects).await {
        eprintln!("Failed to save cache: {e}");
    }

    Ok(ApiResponse {
        data: objects_to_notify,
    })
}
