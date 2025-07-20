mod anytype;

use anytype::parser::fetch;

use dotenv::dotenv;

#[derive(Clone)]
pub struct Url(pub String);

#[derive(Clone)]
pub struct Token(pub String);

#[tokio::main]
async fn main() {
    dotenv().ok();

    let anytype_url = Url(std::env::var("ANYTYPE_URL").expect("ANYTYPE_URL must be set in .env."));
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));

    match fetch(&anytype_url.0, &anytype_token.0).await {
        Ok(data) => println!("Response parsed successfully"),
        Err(e) => println!("Error: {e}"),
    }
}
