use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use tokio::fs::remove_file;

use reqwest::{Response, header::HeaderMap};

use crate::{Token, Url, matrix::api};

#[derive(Clone)]
pub struct User(pub String);

#[derive(Debug, Clone)]
pub struct RoomId(pub String);

#[derive(Clone)]
pub struct Password(pub String);

/// Клиент для взаимодействия с api матрикса
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
        let file = File::open("assets/tokens.txt")?;

        let mut reader = BufReader::new(file).lines();

        let access_t;
        let refresh_t;

        if let Some(first_line) = reader.next() {
            access_t = first_line?;
        } else {
            return Err("File is empty".into());
        }

        if let Some(second_line) = reader.next() {
            refresh_t = second_line?;
        } else {
            return Err("File has only one line".into());
        }

        Ok(Self {
            host: host_val,
            client: reqwest::Client::builder().build()?,
            access_token: Token(access_t),
            refresh_token: Token(refresh_t),
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
            fs::create_dir("assets/")?;
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

    /// Фукнция для отправки put запроса на api матрикса
    pub async fn put<T: serde::Serialize>(
        &self,
        path: &str,
        headers: HeaderMap,
        body: T,
    ) -> Result<Response, Box<dyn Error>> {
        let mut url = self.host.0.clone();
        url.push_str(path);

        match self
            .client
            .put(url.trim())
            .headers(headers)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(message) => Err(Box::new(message)),
        }
    }

    /// Взаимодействие с auth частью api матрикса
    pub fn auth(&self) -> api::auth::Auth {
        api::auth::Auth::new(self.clone())
    }

    /// Взаимодействие с room частью api матрикса
    pub fn room(&self) -> api::room::Room {
        api::room::Room::new(self.clone())
    }
}

/// Функция, которая создаст ```Client``` матрикса с access и refresh токенами. Делает login и
/// сохраняет полученные токены
async fn set_client_with_login(matrix_server: Url) -> Result<Client, Box<dyn Error>> {
    let user_name = User(std::env::var("MATRIX_USER").expect("MATRIX_USER must be set in .env."));
    let password =
        Password(std::env::var("MATRIX_PASSWORD").expect("MATRIX_PASSWORD must be set in .env."));

    let mut matrix_client = Client::new(matrix_server)?;

    matrix_client = matrix_client.auth().login(user_name, password).await?;

    match matrix_client.save_tokens() {
        Ok(_) => println!("Matrix client set"),
        Err(message) => return Err(message),
    };

    Ok(matrix_client)
}

/// Функция, которая создаст ```Client``` матрикса с access и refresh токенами. Берет токены из
/// файла и проверяет их валидность
async fn load_client_from_file(matrix_server: &Url) -> Result<Client, Box<dyn Error>> {
    let mut matrix_client: Client = Client::new_from_file(matrix_server.clone())?;

    if matrix_client.auth().who_am_i().await.is_ok() {
        println!("Matrix client set");
        return Ok(matrix_client);
    }

    matrix_client = matrix_client.auth().refresh().await?;

    match matrix_client.save_tokens() {
        Ok(_) => println!("Matrix client set"),
        Err(message) => return Err(message),
    };
    Ok(matrix_client)
}

/// Функция, которая создаст ```Client``` матрикса с access и refresh токенами. Либо берет токены
/// из файла "assets/tokens.txt", либо делает login
pub async fn set_client(matrix_server: Url) -> Result<Client, Box<dyn Error>> {
    let matrix_client: Client;
    loop {
        if Path::new("assets/tokens.txt").exists() {
            matrix_client = match load_client_from_file(&matrix_server).await {
                Ok(cl) => cl,
                Err(message) => {
                    eprintln!("Warn: {message}");
                    remove_file("assets/tokens.txt").await?;
                    continue;
                }
            };
            break;
        } else {
            matrix_client = set_client_with_login(matrix_server).await?;
            break;
        }
    }

    Ok(matrix_client)
}
