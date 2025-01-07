use std::env;

use surrealdb::Uuid;

use crate::errors::{websocket::WebSocketError, KromerError};

pub fn make_url(uuid: Uuid) -> Result<String, KromerError> {
    let force_insecure = env::var("FORCE_INSECURE")
        .map_err(|_| KromerError::WebSocket(WebSocketError::ServerConfigError))?;
    let schema = if force_insecure == "true" {
        "ws"
    } else {
        "wss"
    };
    let server_url = env::var("PUBLIC_URL")
        .map_err(|_| KromerError::WebSocket(WebSocketError::ServerConfigError))?;

    Ok(format!("{schema}://{server_url}/api/v1/ws/gateway/{uuid}"))
}
