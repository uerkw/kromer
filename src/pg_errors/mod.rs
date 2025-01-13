use actix_web::{error, http::StatusCode, HttpResponse};

pub mod address;
pub mod auth;
pub mod generic;
pub mod internal;
pub mod name;
pub mod routes;
pub mod transaction;

#[derive(Debug, thiserror::Error)]
pub enum PgKromerError {
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

pub trait KromerErrorHelper {
    fn error_type(&self) -> &str;
}

impl error::ResponseError for PgKromerError {
    fn status_code(&self) -> StatusCode {
        match self {
            PgKromerError::Generic(e) => e.status_code(),
            PgKromerError::Address(e) => e.status_code(),
            PgKromerError::Name(e) => e.status_code(),
            PgKromerError::Transaction(e) => e.status_code(),
            PgKromerError::Routes(e) => e.status_code(),
            PgKromerError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PgKromerError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PgKromerError::Auth(e) => e.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            PgKromerError::Generic(e) => e.error_response(),
            PgKromerError::Address(e) => e.error_response(),
            PgKromerError::Name(e) => e.error_response(),
            PgKromerError::Transaction(e) => e.error_response(),
            PgKromerError::Routes(e) => e.error_response(),
            PgKromerError::Internal(e) => e.error_response(),
            PgKromerError::Database(e) => {
                let error = ErrorResponse {
                    ok: false,
                    error: "database_error",
                    message: Some(e.to_string()),
                    authed: None,
                };

                HttpResponse::build(self.status_code()).json(error)
            }
            PgKromerError::Auth(e) => e.error_response(),
        }
    }
}