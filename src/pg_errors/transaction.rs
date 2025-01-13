use actix_web::{error, http::StatusCode, HttpResponse};
use thiserror::Error;

use super::{ErrorResponse, KromerErrorHelper};

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Transaction not found")]
    NotFound,

    #[error("Transactions disabled")]
    Disabled,

    #[error("Transaction conflict for parameter {0}")]
    Conflict(String),
}

impl error::ResponseError for TransactionError {
    fn status_code(&self) -> StatusCode {
        match self {
            TransactionError::InsufficientFunds => StatusCode::FORBIDDEN,
            TransactionError::NotFound => StatusCode::NOT_FOUND,
            TransactionError::Disabled => StatusCode::LOCKED,
            TransactionError::Conflict(_) => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error = ErrorResponse {
            ok: false,
            error: self.error_type(),
            message: Some(self.to_string()),
            authed: None,
        };
        HttpResponse::build(self.status_code()).json(error)
    }
}

impl KromerErrorHelper for TransactionError {
    fn error_type(&self) -> &str {
        match self {
            TransactionError::InsufficientFunds => "insufficient_funds",
            TransactionError::NotFound => "transaction_not_found",
            TransactionError::Disabled => "transactions_disabled",
            TransactionError::Conflict(_) => "transaction_conflict",
        }
    }
}