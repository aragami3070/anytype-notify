use crate::RequiredTypes;

use serde::{Deserialize, Serialize};
use config::Config;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub required_types: RequiredTypes,
    pub interval_minutes: u64,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config = Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;
        
        Ok(config.try_deserialize()?)
    }
}
