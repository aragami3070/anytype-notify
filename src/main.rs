mod anytype;
mod config;
mod dotenv_vars;
mod matrix;

use crate::{
    anytype::{
        entities::notification::{NotificationType},
        parser::get_anytype_to_matrix_map,
        sentinel::find_objects_to_notify,
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
    let id_map_type = &config.anytype_to_matrix_id_map_type;

    let objects_to_notify =
        match find_objects_to_notify(&anytype_env.url, &anytype_env.token, &config).await {
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

    if objects_to_notify.is_none() {
        println!("No objects to notify");
        return;
    }

    // Check if there are objects to notify
    let objects_to_notify = objects_to_notify.unwrap();
    println!(
        "Found {} objects to notify",
        objects_to_notify.objects.len()
    );

    // Create and send notifications for all objects
    for object in objects_to_notify.objects {
        match object.notification_type {
            NotificationType::New => {
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
                        eprintln!("Error sending new notification: {err}");
                        process::exit(1);
                    }
                }
            }
            NotificationType::Unassigned | NotificationType::UpcomingDeadline => {
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
                        eprintln!("Error sending renotify notification: {err}");
                        process::exit(1);
                    }
                }
            }
        }
    }
}
