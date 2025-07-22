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

    pub async fn send_message(
        &self,
        room_id: &RoomId,
        device_id: &DeviceId,
        text: String,
    ) -> Result<EventId, Box<dyn Error>> {
        let path = format!(
            "/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
            room_id.0, device_id.0
        );

        let mut headers = HeaderMap::new();

        headers.insert("Accept", "application/json".parse()?);
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.client.get_access_token().0).parse()?,
        );

        let body = MessageBody {
            body: text,
            msgtype: "m.text".to_owned(),
        };

        let response = self
            .client
            .put(path.trim(), headers, body)
            .await?;

		let result = response.json::<EventId>().await?;

        Ok(result)
    }
}
