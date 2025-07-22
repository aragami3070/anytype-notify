use std::error::Error;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::matrix::{api::auth::DeviceId, client::{Client, RoomId}};


#[derive(Serialize)]
pub struct MessageBody {
    pub body: String,
    pub msgtype: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct EventId {
    #[serde(rename = "event_id")]
	pub value: String,
}

pub struct Room {
    pub client: Client,
}

impl Room {
    pub fn new(client: Client) -> Self {
        Room { client }
    }
}
