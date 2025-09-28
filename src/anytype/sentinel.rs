use crate::{
    Token, Url,
    anytype::{
        entities::{
            api_response::{AnytypeObject, ApiResponse},
            cache::{AnytypeCache, CachedObject},
            notification::{NotificationObject, NotificationType, Notifications},
        },
        parser::get_anytype_objects,
    },
    config::AppConfig,
};

use std::{
    error::Error,
    fs::{self, File},
    path::Path,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local};

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
        if !objects_to_notify // Check if object is already in the list
            .iter()
            .any(|o| o.id == notification_object.id)
        {
            objects_to_notify.push(notification_object.clone());
        }
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
    if !objects_to_notify // Check if object is already in the list
        .iter()
        .any(|o| o.id == notification_object.id)
    {
        objects_to_notify.push(notification_object.clone());
    }

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

    if notify_flag
        && !objects_to_notify // Check if object is already in the list
            .iter()
            .any(|o| o.id == notification_object.id)
    {
        objects_to_notify.push(notification_object.clone());
    }

    cached_objects.objects.insert(id.to_string(), cached_object);
}

/// Find Anytype objects with creation date after last check and
/// objects that already existed, but need to notification again.
pub async fn find_objects_to_notify(
    anytype_url: &Url,
    anytype_token: &Token,
    config: &AppConfig,
) -> Result<Option<Notifications>, Box<dyn Error>> {
    let cache_path = "assets/cache.json";

    let current_objects = get_anytype_objects(anytype_url, anytype_token).await?;

    // At the first run create initial cache and exit
    if !Path::new(cache_path).exists() {
        println!("Cache not found. Saving current objects and exiting.");
        set_initial_cache(current_objects, cache_path).await?;
        return Ok(None);
    }

    let mut cached_objects = load_from_cache(cache_path).await?;

    let mut objects_to_notify: Vec<NotificationObject> = Vec::new();

    get_new_objects(
        &current_objects,
        &mut cached_objects,
        &mut objects_to_notify,
    )
    .await?;

    get_objects_for_renotify(
        &current_objects,
        &mut cached_objects,
        &mut objects_to_notify,
        config,
    )
    .await?;

    // Save updated cache
    if let Err(e) = save_to_cache(cache_path, &cached_objects).await {
        eprintln!("Failed to save cache: {e}");
    }

    let objects_to_notify = if objects_to_notify.is_empty() {
        None
    } else {
        Some(Notifications {
            objects: objects_to_notify,
        })
    };

    Ok(objects_to_notify)
}

/// Get new Anytype objects
async fn get_new_objects(
    current_objects: &ApiResponse,
    cached_objects: &mut AnytypeCache,
    objects_to_notify: &mut Vec<NotificationObject>,
) -> Result<(), Box<dyn Error>> {
    // Compare current objects with cached and find unnotified objects with enabled notifications
    for o in &current_objects.data {
        let id = &o.id;
        let notify_flag = o.is_notify_enabled();

        // Create notification content
        let notification_object = NotificationObject::new(o, NotificationType::New)?;

        match cached_objects.objects.get_mut(id) {
            Some(obj) => {
                // Object exists in cache
                process_cached_object(obj, notify_flag, &notification_object, objects_to_notify)
                    .await
            }
            None => {
                // Object doesn't exist in cache
                process_new_object(
                    id,
                    notify_flag,
                    &notification_object,
                    cached_objects,
                    objects_to_notify,
                )
                .await
            }
        }
    }

    Ok(())
}

/// Get Anytype objects that already existed, but need to to notification again.
async fn get_objects_for_renotify(
    current_objects: &ApiResponse,
    cached_objects: &mut AnytypeCache,
    objects_to_notify: &mut Vec<NotificationObject>,
    config: &AppConfig,
) -> Result<(), Box<dyn Error>> {
    // Compare current objects with cached and find objects which need to renotify
    for o in &current_objects.data {
        let id = &o.id;
        let notify_flag = o.is_notify_enabled();

        // if this object not need notify then skip
        if !notify_flag {
            continue;
        }

        // Create notification content
        if let Some(obj) = cached_objects.objects.get_mut(id) {
            if obj.notified {
                check_unassigned(o, obj, objects_to_notify, config).await?;
                check_deadline_upcoming(o, obj, objects_to_notify, config).await?;
            }
        }
    }

    Ok(())
}

async fn check_unassigned(
    object: &AnytypeObject,
    cached_object: &mut CachedObject,
    objects_to_notify: &mut Vec<NotificationObject>,
    config: &AppConfig,
) -> Result<(), Box<dyn Error>> {
    let interval_days = config.renotify_interval.unassigned;
    let days_to_sec: u64 = 24 * 60 * 60;
    let time_now = SystemTime::now();

    let notification_object = NotificationObject::new(object, NotificationType::Unassigned)?;

    if time_now.duration_since(cached_object.notified_in_time)?
        >= Duration::from_secs(interval_days * days_to_sec)
        && notification_object.assignee.is_empty()
    {
        process_renotify_object(cached_object, &notification_object, objects_to_notify).await
    }

    Ok(())
}

async fn check_deadline_upcoming(
    object: &AnytypeObject,
    cached_object: &mut CachedObject,
    objects_to_notify: &mut Vec<NotificationObject>,
    config: &AppConfig,
) -> Result<(), Box<dyn Error>> {
    let interval_days = config.renotify_interval.deadline_upcoming;
    let time_now = Local::now();

    if let Some(due_date_str) = object
        .properties
        .iter()
        .find(|p| p.key == "due_date")
        .and_then(|p| p.date.as_deref())
    {
        if let Ok(due_date) = DateTime::parse_from_rfc3339(due_date_str) {
            let time_diff = due_date.with_timezone(&Local) - time_now;

            if time_diff.num_seconds() >= 0 && time_diff.num_days() as u64 <= interval_days {
                let notification_object =
                    NotificationObject::new(object, NotificationType::UpcomingDeadline)?;
                process_renotify_object(cached_object, &notification_object, objects_to_notify)
                    .await
            }
        }
    }

    Ok(())
}
