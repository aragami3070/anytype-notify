mod anytype;
mod config;
mod matrix;

use crate::anytype::sentinel::find_new_objects;
use crate::matrix::client::{RoomId, set_client};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Clone)]
pub struct Url(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequiredTypes {
    pub types: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let anytype_url = Url(std::env::var("ANYTYPE_URL").expect("ANYTYPE_URL must be set in .env."));

    let new_objects = match find_new_objects(&anytype_url).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Errot: find_new_objects failed: {e:#}");
            process::exit(1);
        }
    };

    println!("Found {} new objects", new_objects.data.len());
    if new_objects.data.is_empty() {
        println!("Nothing to do, exiting.");
        return;
    }

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

    for o in &new_objects.data {
        let name = &o.name;
        let snippet = o.snippet.as_deref().unwrap_or("<no snippet>");

        let date = o
            .properties
            .iter()
            .find(|p| p.key == "created_date")
            .and_then(|p| p.date.as_deref())
            .unwrap_or("<no creation date>");

        let message = format!(
            "Обнаружена новая задача:\n{name}\n\nДетали: {snippet}\n\nДата создания: {date}"
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
        println!("name: {name}");
        println!("snippet: {snippet}");
        println!("creation date: {date}");
        println!();
    }
}
