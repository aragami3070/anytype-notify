use crate::{
    Token, Url,
    anytype::entities::cache::{AnytypeCache, CachedObject},
    anytype::entities::notification::{NotificationObject, Notifications},
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
pub async fn find_new_objects(anytype_url: &Url, anytype_token: &Token) -> Result<Notifications, Box<dyn Error>> {
    let cache_path = "assets/cache.json";

    let current_objects = get_anytype_objects(anytype_url, anytype_token).await?;

    if !Path::new(cache_path).exists() {
        println!("Cache not found. Saving current objects and exiting.");
        let mut initial_cache = AnytypeCache::default();

        for o in &current_objects.data {
            let id = &o.id;
            let notify_flag = o.is_notify_enabled();
            let assignee = o.assignee();
            let proposed_by = o.proposed_by();
            initial_cache.objects.insert(
                id.clone(),
                CachedObject {
                    notify: notify_flag,
                    notified: notify_flag,
                    assignee,
                    proposed_by,
                },
            );
        }

        if let Err(e) = save_to_cache(cache_path, &initial_cache).await {
            eprintln!("Failed to save initial cache: {e}");
        }

        return Ok(Notifications { objects: vec![] });
    }

    let mut cached_objects = load_from_cache(cache_path).await?;

    let mut objects_to_notify: Vec<NotificationObject> = Vec::new();

    for o in &current_objects.data {
        let id = &o.id;
        let notify_flag = o.is_notify_enabled();
        let notification_object = NotificationObject {
            name: o.name.clone(),
            snippet: o.snippet.as_deref().unwrap_or("<no snippet>").to_string(),
            due_date: o.due_date(),
            creation_date: o.creation_date(),
            proposed_by: o.proposed_by(),
            assignee: o.assignee(),
        };

        match cached_objects.objects.get_mut(id) {
            Some(obj) => {
                // Object exists in cache
                if notify_flag && !obj.notified {
                    objects_to_notify.push(notification_object.clone());
                    obj.notify = true;
                    obj.notified = true;
                    obj.assignee = notification_object.assignee;
                    obj.proposed_by = notification_object.proposed_by;
                } else if !notify_flag {
                    obj.notify = false;
                    obj.notified = false;
                    obj.assignee = notification_object.assignee;
                    obj.proposed_by = notification_object.proposed_by;
                }
            }
            None => {
                // Object doesn't exist in cache
                if notify_flag {
                    objects_to_notify.push(notification_object.clone());
                    cached_objects.objects.insert(
                        id.clone(),
                        CachedObject {
                            notify: true,
                            notified: true,
                            assignee: notification_object.assignee,
                            proposed_by: notification_object.proposed_by,
                        },
                    );
                } else {
                    cached_objects.objects.insert(
                        id.clone(),
                        CachedObject {
                            notify: false,
                            notified: false,
                            assignee: notification_object.assignee,
                            proposed_by: notification_object.proposed_by,
                        },
                    );
                }
            }
        }
    }

    if let Err(e) = save_to_cache(cache_path, &cached_objects).await {
        eprintln!("Failed to save cache: {e}");
    }

    Ok(Notifications {
        objects: objects_to_notify,
    })
}
