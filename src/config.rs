use crate::AnytypeToMatrixIdMapType;

use config::Config;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    // The name of the Anytype object type which contains the "anytype_id" and "matrix_id" properties
    pub anytype_to_matrix_id_map_type: AnytypeToMatrixIdMapType,

    // Interval of checking for new objects
    #[allow(dead_code)]
    pub interval_minutes: i64,

    // Interval of checking for old objects for renotify
	pub interval_days: u64
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config = Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
