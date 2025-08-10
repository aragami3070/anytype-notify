use std::error::Error;

use crate::{Token, Url};

#[derive(Debug, Clone)]
pub struct AnytypeVars {
    pub url: Url,
    pub token: Token,
}

pub fn get_anytype_env_vars() -> Result<AnytypeVars, Box<dyn Error>> {
    let url = Url(std::env::var("ANYTYPE_URL")?); // Anytype space URL
    let token = Token(std::env::var("ANYTYPE_TOKEN")?); // Anytype API token
	
	Ok(AnytypeVars{
		url,
		token
	})
}
