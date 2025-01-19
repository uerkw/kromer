use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::websocket::WebSocketError;

#[derive(Debug, Deserialize, Serialize)]
pub enum NumberOrString {
    Number(serde_json::Number),
    String(String),
}

impl TryFrom<Value> for NumberOrString {
    type Error = WebSocketError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(n) => Ok(NumberOrString::Number(n)),
            Value::String(s) => Ok(NumberOrString::String(s)),
            _ => Err(WebSocketError::IdConversionError),
        }
    }
}
