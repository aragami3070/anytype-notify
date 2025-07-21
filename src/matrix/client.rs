use std::error::Error;

use reqwest::{Response, header::HeaderMap};

use crate::{Token, Url, matrix::api};

#[derive(Clone)]
pub struct Client {
    host: Url,
    client: reqwest::Client,
    access_token: Token,
    refresh_token: Token,
}

impl Client {
    pub fn new(host: Url) -> Result<Client, Box<dyn Error>> {
        Ok(Self {
            host: host,
            client: reqwest::Client::builder().build()?,
            access_token: Token(String::new()),
            refresh_token: Token(String::new()),
        })
    }

    pub fn set_tokens(&mut self, access_token: Token, refresh_token: Token) {
        self.access_token = access_token;
        self.refresh_token = refresh_token;
    }

}
