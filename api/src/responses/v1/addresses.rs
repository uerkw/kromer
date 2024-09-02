use sea_orm::sqlx::types::chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressResponse {
    pub ok: bool,
    pub total: u64,
    pub count: u64,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub address: String,
    pub balance: f32,
    #[serde(rename = "totalin")]
    pub total_in: f32,
    #[serde(rename = "totalout")]
    pub total_out: f32,
    #[serde(rename = "firstseen")]
    pub first_seen: DateTime<FixedOffset>,
}
