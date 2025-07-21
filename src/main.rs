mod anytype;

use anytype::parser::get_anytype_objects;

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
    let raw_required_types =
        std::env::var("REQUIRED_TYPES").expect("REQUIRED_TYPES must be set in .env.");

    // Parse required types
    let required_types = raw_required_types
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let objects = match get_anytype_objects(&anytype_url.0, &anytype_token.0, &required_types).await {
        Ok(data) => {
            data
        }
        Err(message) => {
            println!("Error: {}", message);
            return;
        }
    };

    for o in objects.data {
        println!("{}", o.name);
    }
}
