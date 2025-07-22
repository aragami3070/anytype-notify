use serde::Serialize;

use crate::matrix::{api::auth::DeviceId, client::{Client, RoomId}};


#[derive(Serialize)]
pub struct MessageBody {
    pub body: String,
    pub msgtype: String,
}

pub struct Room {
    pub client: Client,
}

impl Room {
    pub fn new(client: Client) -> Self {
        Room { client }
    }
}
