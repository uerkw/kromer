use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TransactionJson {
    pub id: i64,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: i64,
    pub time: String,
    pub name: Option<String>,
    pub metadata: Option<String>,
    pub sent_metaname: Option<String>,
    pub sent_name: Option<String>,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    #[default]
    Unknown,
    Mined,
    NamePurchase,
    NameARecord,
    NameTransfer,
    Transfer,
}
