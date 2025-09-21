use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::anytype::entities::api_response::AnytypeObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    New,
    Unassigned,
    UpcomingDeadline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationObject {
    pub name: String,
    pub snippet: String,
    pub creation_date: String,
    pub due_date: String,
    pub proposed_by: Vec<String>,
    pub assignee: Vec<String>,
    pub notification_type: NotificationType,
}

impl NotificationObject {
    pub fn new(
        object: &AnytypeObject,
        notification_type: NotificationType,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: object.name.clone(),
            snippet: object
                .snippet
                .as_deref()
                .unwrap_or("<no snippet>")
                .to_string(),
            due_date: object.due_date(),
            creation_date: object.creation_date(),
            proposed_by: object.proposed_by(),
            assignee: object.assignee(),
            notification_type,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub objects: Vec<NotificationObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnytypeToMatrixIdMap {
    pub map: HashMap<String, String>, // anytype_id -> matrix_id
}
