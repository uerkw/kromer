use std::env;

use surrealdb::Uuid;

use crate::errors::{websocket::WebSocketError, KromerError};

pub fn make_url(uuid: Uuid) -> Result<String, KromerError> {
    let schema = "ws";
    let host =
        env::var("HOST").map_err(|_| KromerError::WebSocket(WebSocketError::ServerConfigError))?;
    let port =
        env::var("PORT").map_err(|_| KromerError::WebSocket(WebSocketError::ServerConfigError))?;

    let server_url = format!("{host}:{port}");

    Ok(format!("{schema}://{server_url}/ws/gateway/{uuid}"))
}
