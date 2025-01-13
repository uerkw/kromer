use crate::entities::name;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NameResponse {
    pub ok: bool,
    pub total: u64,
    pub count: u64,
    pub names: Vec<Name>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingularNameResponse {
    pub ok: bool,
    pub name: Name,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameAvailabilityResponse {
    pub ok: bool,
    pub available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameCostResponse {
    pub ok: bool,
    #[serde(rename = "name_cost")]
    pub cost: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub owner: String,
    pub registered: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<NaiveDateTime>,
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl From<name::Model> for Name {
    fn from(name: name::Model) -> Self {
        Name {
            name: name.name,
            owner: name.owner,
            registered: name.registered,
            updated: name.updated,
            metadata: name.metadata,
        }
    }
}