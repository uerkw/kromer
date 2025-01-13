use actix_web::{error, http::StatusCode, HttpResponse};
use thiserror::Error;

use super::{ErrorResponse, KromerErrorHelper};

#[derive(Error, Debug)]
pub enum NameError {
    #[error("Name {0} not found")]
    NameNotFound(String),

    #[error("Name {0} is already taken")]
    NameTaken(String),

    #[error("You are not the owner of name {0}")]
    NotNameOwner(String),

    #[error("Insufficient balance to purchase name")]
    InsufficientBalance,
}

impl error::ResponseError for NameError {
    fn status_code(&self) -> StatusCode {
        match self {
            NameError::NameNotFound(_) => StatusCode::NOT_FOUND,
            NameError::NameTaken(_) => StatusCode::CONFLICT,
            NameError::NotNameOwner(_) => StatusCode::FORBIDDEN,
            NameError::InsufficientBalance => StatusCode::IM_A_TEAPOT, // Really dont know what to put here instead of 418.
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

impl KromerErrorHelper for NameError {
    fn error_type(&self) -> &str {
        match self {
            NameError::NameNotFound(_) => "name_not_found",
            NameError::NameTaken(_) => "name_taken",
            NameError::NotNameOwner(_) => "not_name_owner",
            NameError::InsufficientBalance => "insufficient_balance",
        }
    }
}