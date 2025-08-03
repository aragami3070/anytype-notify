use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedObject {
    pub notify: bool,
    pub notified: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnytypeCache {
    pub objects: HashMap<String, CachedObject>,
}
