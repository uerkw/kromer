use thiserror::Error;
use actix_web::{error, HttpResponse};

use super::{ErrorResponse, KromerErrorHelper};

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("Invalid parameter {0}")]
    InvalidParameter(String),

    #[error("Missing parameter {0}")]
    MissingParameter(String),
}

impl error::ResponseError for GenericError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = ErrorResponse {
            ok: false,
            error: self.error_type(),
            message: Some(self.to_string()),
        };

        HttpResponse::build(self.status_code()).json(message)
    }
}

impl KromerErrorHelper for GenericError {
    fn error_type(&self) -> &str {
        match self {
            GenericError::InvalidParameter(_) => "invalid_parameter",
            GenericError::MissingParameter(_) => "missing_parameter",
        }
    }
}