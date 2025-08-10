use std::error::Error;

use crate::{Token, Url, matrix::client::RoomId};

#[derive(Debug, Clone)]
pub struct AnytypeVars {
    pub url: Url,
    pub token: Token,
}

#[derive(Debug, Clone)]
pub struct MatrixVars {
    pub server: Url,
    pub room_id: RoomId,
}

pub fn get_anytype_env_vars() -> Result<AnytypeVars, Box<dyn Error>> {
    let url = Url(std::env::var("ANYTYPE_URL")?); // Anytype space URL
    let token = Token(std::env::var("ANYTYPE_TOKEN")?); // Anytype API token

    Ok(AnytypeVars { url, token })
}

pub fn get_matrix_env_vars() -> Result<MatrixVars, Box<dyn Error>> {
    let server = Url(std::env::var("MATRIX_SERVER")?);
    let room_id = RoomId(std::env::var("MATRIX_ROOM_ID")?);

    Ok(MatrixVars { server, room_id })
}

