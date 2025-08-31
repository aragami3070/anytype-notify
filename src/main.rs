mod anytype;
mod config;
mod dotenv_vars;
mod matrix;

use crate::{
    anytype::{parser::get_anytype_to_matrix_map, sentinel::find_objects_needed_notify},
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

    let new_objects = match find_objects_needed_notify(&anytype_env.url, &anytype_env.token).await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: find_new_objects failed: {err:#}");
            process::exit(1);
        }
    };

    println!("Found {} new objects", new_objects.objects.len());
    // Check if there are no new objects
    if new_objects.objects.is_empty() {
        println!("Nothing to do, exiting.");
        return;
    }

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

    // Create and send notifications for all new objects
    for object in new_objects.objects {
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
}
