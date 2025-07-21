mod anytype;
mod matrix;

use serde::{Deserialize, Serialize};
use std::process;

use anytype::parser::fetch;

use dotenv::dotenv;

use crate::matrix::client;

#[derive(Clone)]
pub struct Url(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token(pub String);

#[derive(Clone)]
pub struct User(pub String);

#[derive(Clone)]
pub struct Password(pub String);

#[tokio::main]
async fn main() {
    dotenv().ok();

    let anytype_url = Url(std::env::var("ANYTYPE_URL").expect("ANYTYPE_URL must be set in .env."));
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));

    match fetch(&anytype_url.0, &anytype_token.0).await {
        Ok(_) => println!("Response parsed successfully"),
        Err(e) => println!("Error: {e}"),
    }

    let matrix_server =
        Url(std::env::var("MATRIX_SERVER").expect("MATRIX_SERVER must be set in .env."));

    let user = User(std::env::var("MATRIX_USER").expect("MATRIX_USER must be set in .env."));
    let password =
        Password(std::env::var("MATRIX_PASSWORD").expect("MATRIX_PASSWORD must be set in .env."));

    let mut matrix_client = match client::Client::new(matrix_server) {
        Ok(cl) => cl,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    matrix_client = match matrix_client.auth().login(user, password).await {
        Ok(m) => m,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    match matrix_client.save_tokens() {
        Ok(r) => println!("{r}"),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    }
}
