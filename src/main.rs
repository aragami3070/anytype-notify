mod anytype;
mod config;
mod dotenv_vars;
mod matrix;

use crate::{
    anytype::{
        entities::notification::Notifications,
        parser::get_anytype_to_matrix_map,
        sentinel::{Days, find_objects_needed_notify},
    },
    config::AppConfig,
    matrix::{client::set_client, messages},
};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Clone)]
pub struct Url(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnytypeToMatrixIdMapType(pub String);

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env

    let anytype_env = dotenv_vars::get_anytype_env_vars().unwrap_or_else(|err| {
        println!("Error: ANYTYPE_TOKEN and ANYTYPE_TOKEN must be set in .env\nDetails: {err}");
        process::exit(1);
    });

    // Load config from config.toml
    let config = AppConfig::from_file("config.toml").unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });

    // Anytype object type which contains the "anytype_id" and "matrix_id" properties
    let id_map_type = config.anytype_to_matrix_id_map_type;
    let days: Days = config.interval_days;

    let (new_objects, renotify_objects) =
        match find_objects_needed_notify(&anytype_env.url, &anytype_env.token, days).await {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: find_new_objects failed: {err:#}");
                process::exit(1);
            }
        };

    // Get mapping for finding matrix user ids by anytype space member ids
    let matrix_id_map =
        match get_anytype_to_matrix_map(&anytype_env.url, &anytype_env.token, &id_map_type.0).await
        {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: can not get anytype to matrix id mapping: {err:#}");
                process::exit(1);
            }
        };

    let matrix_env = dotenv_vars::get_matrix_env_vars().unwrap_or_else(|err| {
        println!("Error: MATRIX_SERVER and MATRIX_ROOM_ID must be set in .env\nDetails: {err}");
        process::exit(1);
    });

    let matrix_client = match set_client(matrix_env.server).await {
        Ok(cl) => cl,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

    let device_id = match matrix_client.auth().who_am_i().await {
        Ok(me) => me,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
    .device_id;

    // Check if there are new objects
    let new_notifications = if new_objects.is_some() {
        new_objects.unwrap()
    } else {
        println!("Not new objects");
        Notifications { objects: vec![] }
    };
    if !new_notifications.objects.is_empty() {
        println!("Found {} new objects", new_notifications.objects.len());
    }

    // Create and send notifications for all new objects
    for object in new_notifications.objects {
        match messages::send_message(
            object,
            &matrix_id_map,
            &matrix_client,
            &matrix_env.room_id,
            &device_id,
        )
        .await
        {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error: {err}");
                process::exit(1);
            }
        }
    }

    // Check if there are renotify objects
    let renotifications = if renotify_objects.is_some() {
        renotify_objects.unwrap()
    } else {
        println!("Not ignored objects");
        return;
    };

    println!("Found {} ignored objects", renotifications.objects.len());

    // Create and send notifications for all renotify objects
    for object in renotifications.objects {
        match messages::send_renotify_message(
            object,
            &matrix_id_map,
            &matrix_client,
            &matrix_env.room_id,
            &device_id,
        )
        .await
        {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error: {err}");
                process::exit(1);
            }
        }
    }
}
