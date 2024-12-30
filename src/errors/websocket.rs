use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::Error as ActixError;

#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("Failed to read the WebSocket payload")]
    PayloadRead,

    #[error("Failed to deserialize JSON in request")]
    JsonParseRead,

    #[error("Failed to create a WebSocket room")]
    RoomCreation,

    #[error("Server configuration issue, contact an admin")]
    ServerConfigError,

    #[error("UUID was not found in server cache")]
    UuidNotFound,
}

impl From<WebSocketError> for ActixError {
    fn from(err: WebSocketError) -> Self {
        InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR).into()
    }
}
