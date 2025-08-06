use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationObject {
    pub name: String,
    pub snippet: String,
    pub creation_date: String,
    pub proposed_by: Vec<String>,
    pub assignee: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub objects: Vec<NotificationObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnytypeToMatrixIdMap {
    pub map: HashMap<String, String>, // anytype_id -> matrix_id
}
