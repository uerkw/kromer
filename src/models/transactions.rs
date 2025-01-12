use serde::{Deserialize, Serialize};

use crate::database::models::transaction;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TransactionJson {
    pub id: i64,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: f64,
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

impl From<transaction::Model> for TransactionJson {
    fn from(transaction: transaction::Model) -> Self {
        Self {
            id: 0,                                    // We dont do incremental IDs, do we give a shit?
            from: Some(transaction.from.to_string()), // TODO: use address actual address instead.
            to: Some(transaction.to.to_string()),     // TODO: use address actual address instead.
            value: transaction.amount,
            time: transaction.timestamp.to_string(),
            name: None, // TODO: Populate this later, maybe with a separate function.
            metadata: transaction.metadata,
            sent_metaname: None, // NOTE: We do not support this, yet.
            sent_name: None,     // NOTE: We do not support this, yet.
            transaction_type: transaction.transaction_type,
        }
    }
}

impl From<TransactionType> for &str {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Unknown => "unknown",
            TransactionType::Mined => "mined",
            TransactionType::NamePurchase => "name_purchase",
            TransactionType::NameARecord => "name_a_record",
            TransactionType::NameTransfer => "name_transfer",
            TransactionType::Transfer => "transfer",
        }
    }
}
