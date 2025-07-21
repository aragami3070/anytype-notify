use std::{error::Error, fs::File, io::Write};

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
    pub fn new(host_val: Url) -> Result<Client, Box<dyn Error>> {
        Ok(Self {
            host: host_val,
            client: reqwest::Client::builder().build()?,
            access_token: Token(String::new()),
            refresh_token: Token(String::new()),
        })
    }

	pub fn save_tokens(&self) {
		let mut token_file = File::create("assets/tokens.txt").expect("Should be able to create file");
		token_file.write_all(format!("{}\n",self.access_token.0).as_bytes()).expect("Should be able to write data");
		token_file.write_all(self.refresh_token.0.as_bytes()).expect("Should be able to write data");
	}

    pub fn set_tokens(&mut self, access_token: Token, refresh_token: Token) {
        self.access_token = access_token;
        self.refresh_token = refresh_token;
    }

    pub async fn post<T: serde::Serialize>(
        &self,
        path: &str,
        headers: HeaderMap,
        body: T,
    ) -> Result<Response, Box<dyn Error>> {
        let mut url = self.host.0.clone();
        url.push_str(path);

        match self
            .client
            .post(url.trim())
            .headers(headers)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(message) => Err(Box::new(message)),
        }
    }

    pub fn auth(&self) -> api::auth::Auth {
        api::auth::Auth::new(self.clone())
    }
}
