use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LoginDetails {
    #[serde(rename = "privatekey")]
    pub private_key: String,
}
