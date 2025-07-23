use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<AnytypeObject>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct AnytypeObject {
    pub archived: bool,
    pub icon: Option<Icon>,
    pub id: String,
    pub layout: String,
    pub name: String,
    pub object: String,
    pub properties: Vec<Property>,
    pub snippet: Option<String>,
    pub space_id: String,
    #[serde(rename = "type")]
    pub type_field: Option<ObjectType>,
}

impl AnytypeObject {
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.properties
            .iter()
            .find(|p| p.key == "created_date")
            .and_then(|p| p.date.as_ref())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Icon {
    pub emoji: Option<String>,
    pub format: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectType {
    pub archived: Option<bool>,
    pub icon: Option<Icon>,
    pub id: String,
    pub key: String,
    pub layout: Option<String>,
    pub name: String,
    pub object: Option<String>,
    pub plural_name: Option<String>,
    pub properties: Option<Vec<Property>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Property {
    pub format: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub object: Option<String>,
    pub select: Option<SelectTag>,

    #[allow(dead_code)]
    pub text: Option<String>,
    pub number: Option<f64>,
    pub checkbox: Option<bool>,
    pub date: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub objects: Option<Vec<String>>,
    pub files: Option<Vec<String>>,
    pub multi_select: Option<Vec<SelectTag>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct SelectTag {
    pub color: String,
    pub id: String,
    pub key: Option<String>,
    pub name: String,
    pub object: Option<String>,
}
