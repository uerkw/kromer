use actix_web::{error, http::StatusCode, HttpResponse};

mod address;
mod generic;
mod name;
mod routes;
mod transaction;
mod internal;
mod auth;

pub use address::*;
pub use generic::*;
pub use name::*;
pub use routes::*;
pub use transaction::*;
pub use internal::*;
pub use auth::*;

#[derive(Debug, thiserror::Error)]
pub enum KromerError {
    #[error("")]
    Generic(#[from] generic::GenericError),

    #[error("")]
    Address(#[from] address::AddressError),

    #[error("")]
    Name(#[from] name::NameError),

    #[error("")]
    Transaction(#[from] transaction::TransactionError),

    #[error("")]
    Database(#[from] sea_orm::DbErr),

    #[error("")]
    Routes(#[from] routes::RoutesError),

    #[error("")]
    Internal(#[from] internal::InternalError),

    #[error("")]
    Auth(#[from] auth::AuthError),
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse<'a> {
    pub ok: bool,
    pub error: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authed: Option<bool>,
}

// NOTE(sov): This might not be needed? We are using an enum for errors, we might be able to just somehow turn that into a camelCase string
/// Extra functions for returning the error type easily
pub trait KromerErrorHelper {
    fn error_type(&self) -> &str;
}

impl error::ResponseError for KromerError {
    fn status_code(&self) -> StatusCode {
        match self {
            KromerError::Generic(e) => e.status_code(),
            KromerError::Address(e) => e.status_code(),
            KromerError::Name(e) => e.status_code(),
            KromerError::Transaction(e) => e.status_code(),
            KromerError::Routes(e) => e.status_code(),
            KromerError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            KromerError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            KromerError::Auth(e) => e.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            KromerError::Generic(e) => e.error_response(),
            KromerError::Address(e) => e.error_response(),
            KromerError::Name(e) => e.error_response(),
            KromerError::Transaction(e) => e.error_response(),
            KromerError::Routes(e) => e.error_response(),
            KromerError::Internal(e) => e.error_response(),
            KromerError::Database(e) => {
                let error = ErrorResponse {
                    ok: false,
                    error: "database_error",
                    message: Some(e.to_string()),
                    authed: None,
                };

                HttpResponse::build(self.status_code()).json(error)
            }
            KromerError::Auth(e) => e.error_response(),
        }
    }
}
