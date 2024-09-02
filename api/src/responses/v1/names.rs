use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NameResponse {
    pub ok: bool,
    pub total: u64,
    pub count: u64,
    pub names: Vec<Name>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub owner: String,
    pub registered: DateTimeWithTimeZone,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTimeWithTimeZone>,
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}