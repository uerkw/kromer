use actix_web::{error, http::StatusCode, HttpResponse};

use super::{ErrorResponse, KromerErrorHelper};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Authentication failed")]
    AuthFailed,
}

impl error::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::AuthFailed => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error = ErrorResponse {
            ok: false,
            authed: Some(false),
            message: Some(self.to_string()),
            error: self.error_type(),
        };
        HttpResponse::build(self.status_code()).json(error)
    }
}

impl KromerErrorHelper for AuthError {
    fn error_type(&self) -> &str {
        match self {
            AuthError::AuthFailed => "auth_failed",
        }
    }
}