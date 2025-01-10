use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DetailedUnpaidResponseRow {
    pub count: i64,
    pub unpaid: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NameJson {
    pub name: String,
    pub owner: Option<String>,
    pub original_owner: Option<String>,
    pub registered: Option<String>,
    pub updated: Option<String>,
    pub transfered: Option<String>,
    pub unpaid: i64,
}
