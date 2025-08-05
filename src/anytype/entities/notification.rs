use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
