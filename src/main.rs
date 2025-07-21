mod anytype;
mod matrix;

use std::process;

use serde::{Deserialize, Serialize};

use anytype::parser::fetch;

use dotenv::dotenv;


#[derive(Clone)]
pub struct Url(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token(pub String);

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

}
