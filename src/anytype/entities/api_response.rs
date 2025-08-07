use chrono::{DateTime, Datelike, Local};
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

    fn month_name(local_time: DateTime<Local>) -> String {
        let month_name = match local_time.month() {
            1 => "января",
            2 => "февраля",
            3 => "марта",
            4 => "апреля",
            5 => "мая",
            6 => "июня",
            7 => "июля",
            8 => "августа",
            9 => "сентября",
            10 => "октября",
            11 => "ноября",
            12 => "декабря",
            _ => "<неизвестно>",
        };

        month_name.to_string()
    }

    pub fn creation_date(&self) -> String {
        let raw = self
            .properties
            .iter()
            .find(|p| p.key == "created_date")
            .and_then(|p| p.date.as_deref());

        match raw {
            Some(date_str) => match DateTime::parse_from_rfc3339(date_str) {
                Ok(dt) => {
                    let local_time = dt.with_timezone(&Local);
                    let day = local_time.day();
                    let year = local_time.year();
                    let month = Self::month_name(local_time);

                    format!(
                        "{day} {month} {year}, {time}",
                        time = local_time.format("%H:%M")
                    )
                }
                Err(_) => format!("Invalid date format: {date_str}"),
            },
            None => "<no creation date>".to_string(),
        }
    }

    pub fn due_date(&self) -> String {
        let raw = self
            .properties
            .iter()
            .find(|p| p.key == "due_date")
            .and_then(|p| p.date.as_deref());

        match raw {
            Some(date_str) => match DateTime::parse_from_rfc3339(date_str) {
                Ok(dt) => {
                    let local_time = dt.with_timezone(&Local);
                    let day = local_time.day();
                    let year = local_time.year();
                    let month = Self::month_name(local_time);

                    format!("{day} {month} {year}")
                }
                Err(_) => format!("Invalid date format: {date_str}"),
            },
            None => "<no deadline>".to_string(),
        }
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
