use actix_web::{error, http::StatusCode, HttpResponse};
use thiserror::Error;

use super::{ErrorResponse, KromerErrorHelper};

#[derive(Error, Debug)]
pub enum RoutesError {
    #[error("Route not found")]
    NotFound,

    #[error("Rate limit hit")]
    RateLimitHit,
}

impl error::ResponseError for RoutesError {
    fn status_code(&self) -> StatusCode {
        match self {
            RoutesError::NotFound => StatusCode::NOT_FOUND,
            RoutesError::RateLimitHit => StatusCode::TOO_MANY_REQUESTS,
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

impl KromerErrorHelper for RoutesError {
    fn error_type(&self) -> &str {
        match self {
            RoutesError::NotFound => "route_not_found",
            RoutesError::RateLimitHit => "rate_limit_hit",
        }
    }
}