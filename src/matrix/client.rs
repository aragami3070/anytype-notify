use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
    process,
};

use tokio::fs::remove_file;

use reqwest::{Response, header::HeaderMap};

use crate::{Token, Url, matrix::api};

#[derive(Clone)]
pub struct User(pub String);

#[derive(Clone)]
pub struct Password(pub String);

#[derive(Clone)]
pub struct Client {
    host: Url,
    client: reqwest::Client,
    access_token: Token,
    refresh_token: Token,
}

impl Client {
    /// Функция создания Client с пустыми токенами
    pub fn new(host_val: Url) -> Result<Client, Box<dyn Error>> {
        Ok(Self {
            host: host_val,
            client: reqwest::Client::builder().build()?,
            access_token: Token(String::new()),
            refresh_token: Token(String::new()),
        })
    }

    /// Функция создания Client с токенами из файла assets/tokens.txt
    pub fn new_from_file(host_val: Url) -> Result<Client, Box<dyn Error>> {
        let file = match File::open("assets/tokens.txt") {
            Ok(f) => f,
            Err(message) => return Err(Box::new(message)),
        };

        let reader = BufReader::new(file).lines();

        let mut tokens: Vec<Token> = Vec::new();
        for line in reader {
            let token_val = match line {
                Ok(t) => Token(t),
                Err(message) => return Err(Box::new(message)),
            };
            tokens.push(token_val);
        }

        Ok(Self {
            host: host_val,
            client: reqwest::Client::builder().build()?,
            access_token: tokens[0].clone(),
            refresh_token: tokens[1].clone(),
        })
    }

    pub fn get_access_token(&self) -> Token {
        self.access_token.clone()
    }

    pub fn get_refresh_token(&self) -> Token {
        self.refresh_token.clone()
    }

    /// Функция сохранения токенов в файл assets/tokens.txt
    pub fn save_tokens(&self) -> Result<&str, Box<dyn Error>> {
        if !Path::new("assets/").exists() {
            match fs::create_dir("assets/") {
                Ok(_) => {}
                Err(message) => {
                    return Err(Box::new(message));
                }
            }
        }

        let mut token_file =
            File::create("assets/tokens.txt").expect("Error: Should be able to create file");
        token_file
            .write_all(format!("{}\n", self.access_token.0).as_bytes())
            .expect("Error: Should be able to write data");
        token_file
            .write_all(self.refresh_token.0.as_bytes())
            .expect("Error: Should be able to write data");

        Ok("Save tokens success")
    }

    pub fn set_tokens(&mut self, access_token: Token, refresh_token: Token) {
        self.access_token = access_token;
        self.refresh_token = refresh_token;
    }

    /// Фукнция для отправки post запроса на api матрикса
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

    /// Фукнция для отправки get запроса на api матрикса
    pub async fn get(&self, path: &str, headers: HeaderMap) -> Result<Response, Box<dyn Error>> {
        let mut url = self.host.0.clone();
        url.push_str(path);

        match self.client.get(url.trim()).headers(headers).send().await {
            Ok(resp) => Ok(resp),
            Err(message) => Err(Box::new(message)),
        }
    }

    /// Взаимодействие с auth частью api матрикса
    pub fn auth(&self) -> api::auth::Auth {
        api::auth::Auth::new(self.clone())
    }
}
