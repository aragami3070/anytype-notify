mod anytype;
mod config;
mod matrix;

use crate::anytype::{
    parser::{find_matrix_user_id, get_anytype_to_matrix_map},
    sentinel::find_new_objects,
};
use crate::config::AppConfig;
use crate::matrix::client::{RoomId, set_client};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Clone)]
pub struct Url(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnytypeToMatrixIdMapType(pub String);

#[tokio::main]
async fn main() {
    dotenv().ok();

    let anytype_url = Url(std::env::var("ANYTYPE_URL").expect("ANYTYPE_URL must be set in .env."));
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));
    let config = AppConfig::from_file("config.toml").unwrap();
    let id_map_type = config.anytype_to_matrix_id_map_type;

    let new_objects = match find_new_objects(&anytype_url, &anytype_token).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: find_new_objects failed: {e:#}");
            process::exit(1);
        }
    };

    println!("Found {} new objects", new_objects.objects.len());
    if new_objects.objects.is_empty() {
        println!("Nothing to do, exiting.");
        return;
    }

    let matrix_id_map =
        match get_anytype_to_matrix_map(&anytype_url, &anytype_token, &id_map_type.0).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error: can not get anytype to matrix id mapping: {e:#}");
                process::exit(1);
            }
        };

    let matrix_server =
        Url(std::env::var("MATRIX_SERVER").expect("MATRIX_SERVER must be set in .env."));

    let matrix_client = match set_client(matrix_server).await {
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

    let room_id =
        RoomId(std::env::var("MATRIX_ROOM_ID").expect("MATRIX_ROOM_ID must be set in .env."));

    for o in &new_objects.objects {
        let name = &o.name;
        let snippet = &o.snippet;
        let date = &o.creation_date;
        let assignee = &o
            .assignee
            .iter()
            .map(|a| find_matrix_user_id(&matrix_id_map, a.as_str()))
            .collect::<Vec<String>>()
            .join(", ");
        let proposed_by = &o
            .proposed_by
            .iter()
            .map(|p| find_matrix_user_id(&matrix_id_map, p.as_str()))
            .collect::<Vec<String>>()
            .join(", ");

        let message = format!(
            "{proposed_by} создал новую задачу:\n{name}\n\nДетали: {snippet}\n\nНазначено: {assignee}\n\nДата создания: {date}"
        );

        match matrix_client
            .room()
            .send_message(&room_id, &device_id, message.to_string())
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
