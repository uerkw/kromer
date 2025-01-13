use actix_web::{error, http::StatusCode, HttpResponse};
use thiserror::Error;

use super::{ErrorResponse, KromerErrorHelper};

#[derive(Error, Debug)]
pub enum AddressError {
    #[error("Address {0} not found")]
    NotFound(String),

    #[error("Authentication failed")]
    AuthFailed,
}

impl error::ResponseError for AddressError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AddressError::NotFound(_) => StatusCode::NOT_FOUND,
            AddressError::AuthFailed => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let message = ErrorResponse {
            ok: false,
            error: self.error_type(),
            message: Some(self.to_string()),
            authed: None,
        };

        HttpResponse::build(self.status_code()).json(message)
    }
}

impl KromerErrorHelper for AddressError {
    fn error_type(&self) -> &str {
        match self {
            AddressError::NotFound(_) => "address_not_found",
            AddressError::AuthFailed => "auth_failed",
        }
    }
}