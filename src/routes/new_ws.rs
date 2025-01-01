use std::str::FromStr;

use actix::Actor;
use actix_web::{
    route,
    web::{self, Data},
    HttpRequest, Responder,
};

use actix_ws::{CloseCode, CloseReason, Message};
use surrealdb::Uuid;

use crate::{
    errors::{websocket::WebSocketError, KromerError},
    ws::{
        actors::session::WebSocketSession,
        types::{
            actor_message::{CloseWebSocket, GetActiveSessions, SetCacheConnection},
            session::KromerAddress,
        },
    },
    AppState,
};

pub struct WebSocketInitData {
    _privatekey: Option<String>,
    _name: Option<String>,
}

#[route("/gateway/{token}", method = "GET")]
pub async fn payload_ws(
    req: HttpRequest,
    body: web::Payload,
    state: Data<AppState>,
    token: web::Path<String>,
) -> Result<impl Responder, KromerError> {
    let token = token.into_inner();
    let _ws_manager = (&state.new_ws_manager).clone();
    let _session_id =
        Uuid::from_str(&token).map_err(|_| KromerError::WebSocket(WebSocketError::UuidNotFound));

    let (response, mut _session, mut _msg_stream) = actix_ws::handle(&req, body)
        .or_else(|_| Err(WebSocketError::RoomCreation))?;

    let address = Some(KromerAddress::from_string("guest".to_string()));
    let token_uuid = Uuid::from_str(&token)
        .or_else(|_| Err(WebSocketError::UuidNotFound))?;

    let wrapped_ws_session = WebSocketSession::new(
        token_uuid,
        address,
        Some("privatekey".to_string()),
        _session,
        _ws_manager.clone(),
    );

    let ws_actor_addr = wrapped_ws_session.start();
    _ws_manager.send(SetCacheConnection(token_uuid, ws_actor_addr)).await;

    let thread_ws_manager = _ws_manager.clone();

    actix_web::rt::spawn(async move {
        let get_active_sessions_msg = GetActiveSessions;
        let active_sessions = thread_ws_manager.send(get_active_sessions_msg).await;
        tracing::debug!(
            "[SPAWNED_WS_THREAD] Active Sessions Before Close: {:?}",
            active_sessions
        );

        let mut close_reason: CloseReason = CloseReason {
            code: CloseCode::Normal,
            description: Some("WebSocket Closed".to_string()),
        };
        while let Some(Ok(msg)) = _msg_stream.recv().await {
            match msg {
                Message::Close(reason) => {
                    close_reason = reason.unwrap_or_else(|| close_reason);
                    tracing::debug!(
                        "[SPAWNED_WS_THREAD] Client WS Closed with Code: {:?}, Description: {:?}",
                        close_reason.code,
                        close_reason.description
                    );
                    break;
                }
                Message::Text(msg) => tracing::debug!("[SPAWNED_WS_THREAD] Got text, msg: {msg}"),
                _ => break,
            }
        }

        let actor_close_message = CloseWebSocket(close_reason);
        let _ = cloned_ws_actor_addr.send(actor_close_message).await;
    });

    let cleanup_ws_manager = _ws_manager.clone();
    let get_active_sessions_msg = GetActiveSessions;
    let active_sessions = cleanup_ws_manager.send(get_active_sessions_msg).await;
    tracing::debug!(
        "[SPAWNED_WS_THREAD] Active Sessions Before Open: {:?}",
        active_sessions
    );
    Ok(response)
}
