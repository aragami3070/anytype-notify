mod anytype;
mod config;
mod dotenv_vars;
mod matrix;

use crate::{
    anytype::{
        parser::{find_matrix_user_id, get_anytype_to_matrix_map},
        sentinel::find_new_objects,
    },
    config::AppConfig,
    matrix::client::set_client,
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

    let new_objects = match find_new_objects(&anytype_env.url, &anytype_env.token).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: find_new_objects failed: {e:#}");
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
            Err(e) => {
                eprintln!("Error: can not get anytype to matrix id mapping: {e:#}");
                process::exit(1);
            }
        };

    let matrix_env = dotenv_vars::get_matrix_env_vars().unwrap_or_else(|err| {
        println!("Error: MATRIX_SERVER and MATRIX_ROOM_ID must be set in .env\nDetails: {err}");
        process::exit(1);
    });

    let matrix_client = match set_client(matrix_env.server).await {
        Ok(cl) => cl,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let device_id = match matrix_client.auth().who_am_i().await {
        Ok(me) => me,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    }
    .device_id;

    // Create and send notifications for all new objects
    for object in &new_objects.objects {
        let name = &object.name;
        let snippet = &object.snippet;
        let creation_date = &object.creation_date;
        let due_date = &object.due_date;
        // Get matrix user ids using mapping
        let assignee = &object
            .assignee
            .iter()
            .map(|a| find_matrix_user_id(&matrix_id_map, a.as_str()))
            .collect::<Vec<String>>()
            .join(", ");
        let proposed_by = &object
            .proposed_by
            .iter()
            .map(|p| find_matrix_user_id(&matrix_id_map, p.as_str()))
            .collect::<Vec<String>>()
            .join(", ");

        let message = format!(
            "От {proposed_by} поступила новая задача:\n{name}\n\n{snippet}\n\n{assignee}\n\nДата создания: {creation_date}\nДедлайн: {due_date}"
        );

        match matrix_client
            .room()
            .send_message(&matrix_env.room_id, &device_id, message.to_string())
            .await
        {
            Ok(cl) => cl,
            Err(message) => {
                eprintln!("Error: {message}");
                process::exit(1);
            }
        };
        println!("Notification text:");
        println!("{message}");
        println!();
    }
}
