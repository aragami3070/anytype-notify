use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<AnytypeObject>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Icon {
    pub emoji: Option<String>,
    pub format: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Property {
    pub format: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub object: Option<String>,
    pub select: Option<SelectTag>,

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

#[derive(Debug, Deserialize)]
pub struct SelectTag {
    pub color: String,
    pub id: String,
    pub key: Option<String>,
    pub name: String,
    pub object: Option<String>,
}
