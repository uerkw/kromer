use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub ok: bool,
    pub total: u64,
    pub count: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: f32,
    pub time: DateTimeWithTimeZone,
    pub name: Option<String>,
    pub sent_metaname: Option<String>,
    pub sent_name: Option<String>,
    pub metadata: Option<String>,
}