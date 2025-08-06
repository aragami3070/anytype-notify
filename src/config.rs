use crate::AnytypeToMatrixMapType;

use serde::Deserialize;
use config::Config;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub required_types: AnytypeToMatrixMapType,
	#[allow(dead_code)]
    pub interval_minutes: i64,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config = Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;
        
        Ok(config.try_deserialize()?)
    }
}
