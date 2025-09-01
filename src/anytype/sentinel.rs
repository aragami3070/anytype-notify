use crate::{
    Token, Url,
    anytype::{
        entities::{
            api_response::ApiResponse,
            cache::{AnytypeCache, CachedObject},
            notification::{NotificationObject, Notifications},
        },
        parser::get_anytype_objects,
    },
};

use std::{
    error::Error,
    fs::{self, File},
    path::Path,
    time::{Duration, SystemTime},
};

pub type Days = u64;

/// Cache Anytype objects in a file for find objects to notify in future checks
async fn save_to_cache(path: &str, objects: &AnytypeCache) -> std::io::Result<()> {
    let cache_path = Path::new(path);

    // Create the directory if it doesn't exist
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, objects)?;
    Ok(())
}

/// Load cached Anytype objects from a file
async fn load_from_cache(path: &str) -> Result<AnytypeCache, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let cache: AnytypeCache = serde_json::from_str(&data)?;
    Ok(cache)
}

/// Create initial cache with actual objects at the first run
async fn set_initial_cache(
    current_objects: ApiResponse,
    cache_path: &str,
) -> Result<(), Box<dyn Error>> {
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
                // If notify is enabled, set cached object to already notified for the first run
                notified: notify_flag,
                assignee,
                proposed_by,
                notified_in_time: SystemTime::now(),
            },
        );
    }

    if let Err(e) = save_to_cache(cache_path, &initial_cache).await {
        eprintln!("Failed to save initial cache: {e}");
    }

    Ok(())
}

async fn process_cached_object(
    object: &mut CachedObject,
    notify_flag: bool,
    notification_object: &NotificationObject,
    objects_to_notify: &mut Vec<NotificationObject>,
) {
    if notify_flag && !object.notified {
        // Object is not already notified and need notification
        objects_to_notify.push(notification_object.clone());
        object.notify = true;
        object.notified = true;
    } else if !notify_flag {
        // Object has disabled notifications
        object.notify = false;
        object.notified = false;
    }

    // Update the other fields
    object.assignee = notification_object.assignee.clone();
    object.proposed_by = notification_object.proposed_by.clone();
}

async fn process_renotify_object(
    object: &mut CachedObject,
    notification_object: &NotificationObject,
    objects_to_notify: &mut Vec<NotificationObject>,
) {
    // Object is need renotification
    objects_to_notify.push(notification_object.clone());

    // Update the other fields
    object.assignee = notification_object.assignee.clone();
    object.proposed_by = notification_object.proposed_by.clone();
    object.notified_in_time = SystemTime::now();
}

async fn process_new_object(
    id: &str,
    notify_flag: bool,
    notification_object: &NotificationObject,
    cached_objects: &mut AnytypeCache,
    objects_to_notify: &mut Vec<NotificationObject>,
) {
    let cached_object = CachedObject {
        notify: notify_flag,
        notified: notify_flag,
        assignee: notification_object.assignee.clone(),
        proposed_by: notification_object.proposed_by.clone(),
        notified_in_time: SystemTime::now(),
    };

    if notify_flag {
        objects_to_notify.push(notification_object.clone());
    }

    cached_objects.objects.insert(id.to_string(), cached_object);
}

/// Find Anytype objects with creation date after last check
pub async fn find_objects_needed_notify(
    anytype_url: &Url,
    anytype_token: &Token,
    days: Days,
) -> Result<(Option<Notifications>, Option<Notifications>), Box<dyn Error>> {
    let cache_path = "assets/cache.json";

    let current_objects = get_anytype_objects(anytype_url, anytype_token).await?;

    // At the first run create initial cache and exit
    if !Path::new(cache_path).exists() {
        println!("Cache not found. Saving current objects and exiting.");
        set_initial_cache(current_objects, cache_path).await?;
        return Ok((None, None));
    }

    let mut cached_objects = load_from_cache(cache_path).await?;

    let new_objects: Vec<NotificationObject> =
        get_new_objects(&current_objects, &mut cached_objects).await?;

    let renotify_objects: Vec<NotificationObject> =
        get_objects_for_renotify(&current_objects, &mut cached_objects, days).await?;

    // Save updated cache
    if let Err(e) = save_to_cache(cache_path, &cached_objects).await {
        eprintln!("Failed to save cache: {e}");
    }

    let renotifications = if renotify_objects.is_empty() {
        None
    } else {
        Some(Notifications {
            objects: renotify_objects,
        })
    };

    let new_notifications = if new_objects.is_empty() {
        None
    } else {
        Some(Notifications {
            objects: new_objects,
        })
    };

    Ok((new_notifications, renotifications))
}

/// Get new Anytype objects
async fn get_new_objects(
    current_objects: &ApiResponse,
    cached_objects: &mut AnytypeCache,
) -> Result<Vec<NotificationObject>, Box<dyn Error>> {
    let mut objects_to_notify: Vec<NotificationObject> = Vec::new();

    // Compare current objects with cached and find unnotified objects with enabled notifications
    for o in &current_objects.data {
        let id = &o.id;
        let notify_flag = o.is_notify_enabled();

        // Create notification content
        let notification_object = NotificationObject::new(o)?;

        match cached_objects.objects.get_mut(id) {
            Some(obj) => {
                // Object exists in cache
                process_cached_object(
                    obj,
                    notify_flag,
                    &notification_object,
                    &mut objects_to_notify,
                )
                .await
            }
            None => {
                // Object doesn't exist in cache
                process_new_object(
                    id,
                    notify_flag,
                    &notification_object,
                    cached_objects,
                    &mut objects_to_notify,
                )
                .await
            }
        }
    }
    Ok(objects_to_notify)
}

/// Get new Anytype objects
async fn get_objects_for_renotify(
    current_objects: &ApiResponse,
    cached_objects: &mut AnytypeCache,
    days: Days,
) -> Result<Vec<NotificationObject>, Box<dyn Error>> {
    let mut objects_to_notify: Vec<NotificationObject> = Vec::new();
    let days_to_sec: u64 = 24 * 60 * 60;

    // Compare current objects with cached and find objects which need to renotify
    for o in &current_objects.data {
        let id = &o.id;
        let notify_flag = o.is_notify_enabled();

        // if this object not need notify then skip
        if !notify_flag {
            continue;
        }

        // Create notification content
        let notification_object = NotificationObject::new(o)?;
        let time_now = SystemTime::now();

        match cached_objects.objects.get_mut(id) {
            Some(obj) => {
                if time_now.duration_since(obj.notified_in_time)?
                    >= Duration::from_secs(days * days_to_sec)
                    && notification_object.assignee.is_empty()
                {
                    process_renotify_object(obj, &notification_object, &mut objects_to_notify).await
                }
            }
            None => {}
        }
    }
    Ok(objects_to_notify)
}
