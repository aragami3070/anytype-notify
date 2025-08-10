use crate::{Token, Url};

#[derive(Debug, Clone)]
pub struct AnytypeVars {
    pub url: Url,
    pub token: Token,
}
