use crate::{Password, Token, User, matrix::client::Client};


pub struct Auth {
    pub client: Client,
}

impl Auth {
    pub fn new(client: Client) -> Self {
        Auth { client }
    }

}
