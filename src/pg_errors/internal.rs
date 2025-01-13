use actix_web::{error, http::StatusCode, HttpResponse};

use super::{ErrorResponse, KromerErrorHelper};

#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    #[error("Argon2 error: {0}")]
    Argon2(String), // Cant use #[from] here...
}

impl error::ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            ok: false,
            error: self.error_type(),
            message: Some(self.to_string()),
            authed: None,
        };

        HttpResponse::build(self.status_code()).json(response)
    }
}

impl KromerErrorHelper for InternalError {
    fn error_type(&self) -> &str {
        match self {
            InternalError::Argon2(_) => "argon2_error",
        }
    }
}