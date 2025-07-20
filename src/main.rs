mod parser;

use parser::parser::fetch;

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Url(pub String);

#[derive(Debug, Clone)]
pub struct Token(pub String);

#[tokio::main]
async fn main() {
    dotenv().ok();

    let anytype_url = Url(std::env::var("ANYTYPE_URL").expect("ANYTYPE_URL must be set in .env."));
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));

    match fetch(&anytype_url.0, &anytype_token.0).await {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("Error: {}", e),
    }
}
