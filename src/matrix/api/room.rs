use crate::matrix::client::Client;

pub struct Room {
    pub client: Client,
}

impl Room {
    pub fn new(client: Client) -> Self {
        Room { client }
    }
    
}
