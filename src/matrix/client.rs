
use reqwest::{Response, header::HeaderMap};

use crate::{Token, Url};

#[derive(Clone)]
pub struct Client {
    host: Url,
    client: reqwest::Client,
    access_token: Token,
    refresh_token: Token,
}
