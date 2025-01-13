use crate::entities::transaction;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub ok: bool,
    pub total: u64,
    pub count: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingularTransactionResponse {
    pub ok: bool,
    pub transaction: Transaction,
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
    pub r#type: TransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "name_purchase")]
    NamePurchase,
    #[serde(rename = "name_a_data")] // I'm gonna kill lemmmy.
    NameMetadataUpdated,
    #[serde(rename = "name_transfer")]
    NameTransfer,
    #[serde(rename = "mined")]
    Mined, // In kromer it's not really mining, more like giving free money.
}

impl TransactionType {
    pub fn indentify(transaction: &transaction::Model) -> TransactionType {
        // THIS IS HORRIBLE AND I HATE IT.
        if transaction.from.is_none() {
            return TransactionType::Mined;
        }

        if transaction.name.is_some() {
            if let Some(to) = &transaction.to {
                match to.as_str() {
                    "name" => return TransactionType::NamePurchase,
                    "metadata" => return TransactionType::NameMetadataUpdated,
                    _ => return TransactionType::NameTransfer,
                }
            }

            return TransactionType::NameTransfer;
        }

        TransactionType::Transfer
    }
}

impl From<transaction::Model> for Transaction {
    fn from(transaction: transaction::Model) -> Self {
        Self {
            r#type: TransactionType::indentify(&transaction),
            id: transaction.id,
            from: transaction.from,
            to: transaction.to,
            value: transaction.value,
            time: transaction.time,
            name: transaction.name,
            sent_metaname: transaction.sent_metaname,
            sent_name: transaction.sent_name,
            metadata: transaction.metadata,
        }
    }
}