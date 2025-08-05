use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse {
    pub data: Vec<AnytypeObject>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    pub fn is_notify_enabled(&self) -> bool {
        self.properties
            .iter()
            .find(|p| p.key == "notify")
            .and_then(|p| p.checkbox)
            .unwrap_or(false)
    }

    pub fn assignee(&self) -> Vec<String> {
        self.properties
            .iter()
            .find(|p| p.key == "assignee")
            .and_then(|p| p.objects.clone())
            .unwrap_or_default()
    }

    pub fn proposed_by(&self) -> Vec<String> {
        self.properties
            .iter()
            .find(|p| p.name == "Proposed by")
            .and_then(|p| p.objects.clone())
            .unwrap_or_default()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Icon {
    pub emoji: Option<String>,
    pub format: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelectTag {
    pub color: String,
    pub id: String,
    pub key: Option<String>,
    pub name: String,
    pub object: Option<String>,
}
