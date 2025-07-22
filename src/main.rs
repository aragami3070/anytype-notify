mod anytype;

use anytype::parser::get_anytype_objects;

use dotenv::dotenv;

#[derive(Clone)]
pub struct Url(pub String);

#[derive(Clone)]
pub struct Token(pub String);

#[derive(Debug, Clone)]
pub struct RequiredTypes {
    pub types: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let anytype_url = Url(std::env::var("ANYTYPE_URL").expect("ANYTYPE_URL must be set in .env."));
    let anytype_token =
        Token(std::env::var("ANYTYPE_TOKEN").expect("ANYTYPE_TOKEN must be set in .env."));
    let raw_required_types =
        std::env::var("REQUIRED_TYPES").expect("REQUIRED_TYPES must be set in .env."); // String

    // Parse required types from String to struct
    let required_types = RequiredTypes {
        types: raw_required_types
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
    };

    let objects = match get_anytype_objects(&anytype_url, &anytype_token, &required_types).await {
        Ok(data) => data,
        Err(message) => {
            eprintln!("Error: {message}");
            return;
        }
    };

    for o in objects.data {
        let name = &o.name;
        let snippet = o.snippet.as_deref().unwrap_or("<no snippet>");

        let date = o
            .properties
            .iter()
            .find(|p| p.key == "created_date")
            .and_then(|p| p.date.as_deref())
            .unwrap_or("<no creation date>");

        println!("name: {name}");
        println!("snippet: {snippet}");
        println!("creation date: {date}");
    }
}
