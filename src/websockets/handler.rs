use crate::{
    errors::{websocket::WebSocketError, KromerError},
    models::{
        error::ErrorResponse,
        websockets::{IncomingWebsocketMessage, OutgoingWebSocketMessage, WebSocketMessageType},
    },
    AppState,
};
use std::{
    pin::pin,
    sync::Arc,
    time::{Duration, Instant},
};

use super::{
    utils::datetime::convert_to_iso_string, wrapped_ws::WrappedWsData, ws_server::WsServerHandle,
};

use crate::websockets::routes::me::get_me as route_get_me;

use actix_web::web::Data;
use actix_ws::AggregatedMessage;
use futures_util::{
    future::{select, Either},
    stream::AbortHandle,
    StreamExt,
};
use serde_json::json;
use surrealdb::{engine::any::Any, Surreal, Uuid};
use tokio::{sync::mpsc, task::JoinHandle, time::interval};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// TODO: 30 seconds for debugging, should probably be around 10 seconds for client timeout in prooduction
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);

pub async fn handle_ws(
    state: Data<AppState>,
    wrapped_ws_data: WrappedWsData,
    ws_server: WsServerHandle,
    mut session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
) -> Result<(), KromerError> {
    let debug_span = tracing::span!(tracing::Level::DEBUG, "WS_HANDLER");
    let _tracing_debug_enter = debug_span.enter();

    let mut last_heartbeat = Instant::now();
    let mut heartbeat_interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    // Internally, the WsServer uses a channel ID to facilitate sending messages.
    let channel_id = match ws_server.connect(conn_tx, wrapped_ws_data.token).await {
        Ok(value) => value,
        Err(_) => return Err(KromerError::WebSocket(WebSocketError::HandshakeError)),
    };

    let msg_stream = msg_stream
        .max_frame_size(64 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let (_keepalive_join_handle, keepalive_abort_handle) =
        spawn_keepalive(ws_server.clone(), channel_id).await;

    let mut msg_stream = pin!(msg_stream);

    // TODO: This state needs to be set globally for Event Subscription Lookups, so perhaps a Mutex is still the preferred option here.
    let mut ws_metadata = wrapped_ws_data.clone();

    let close_reason = loop {
        // Stack pin futures
        let tick = pin!(heartbeat_interval.tick());
        let msg_rx = pin!(conn_rx.recv());

        let messages = pin!(select(msg_stream.next(), msg_rx));

        match select(messages, tick).await {
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
                match msg {
                    AggregatedMessage::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        // Let's ignore pong errors, as they shouldn't really matter here
                        let _ = match session.pong(&bytes).await {
                            Ok(_) => Ok(()),
                            Err(err) => {
                                tracing::error!("Ping failure: {:?}", err);
                                Err(err)
                            }
                        };
                    }

                    AggregatedMessage::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    AggregatedMessage::Text(text) => {
                        // TODO: Better message handling
                        if text.chars().count() > 512 {
                            // TODO: Use error message struct in models
                            let error_msg = json!({
                                "ok": "false",
                                "error": "message_too_long",
                                "message": "Message larger than 512 characters",
                                "type": "error"})
                            .to_string();
                            tracing::info!("Message received was larger than 512 characters");
                            let _ = session.text(error_msg).await;
                        } else {
                            tracing::info!("Message received: {text}");
                            let process_result = process_text_msg(
                                &state.db,
                                &ws_metadata,
                                &ws_server,
                                &mut session,
                                &text,
                            )
                            .await;

                            // If there were updates to the Metadata, we want to do them
                            // TODO: Might need to be a global mutex so subscriptions have access to this as well.
                            if let Ok(Some(new_metadata)) = process_result {
                                ws_metadata = new_metadata;
                            } else if process_result.is_err() {
                                tracing::error!("Error in processing message")
                            }
                        }
                    }

                    AggregatedMessage::Binary(_bin) => {
                        tracing::warn!("unexpected binary message");
                    }

                    AggregatedMessage::Close(reason) => break reason,
                }
            }

            // client WebSocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                tracing::error!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,

            // chat messages received from other room participants
            Either::Left((Either::Right((Some(chat_msg), _)), _)) => {
                let _ = session.text(chat_msg).await;
            }

            // all connection's message senders were dropped
            Either::Left((Either::Right((None, _)), _)) => unreachable!(
                "all connection message senders were dropped; chat server may have panicked"
            ),

            // heartbeat internal tick
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    tracing::info!(
                        "Client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );
                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        };
    };

    keepalive_abort_handle.abort();

    let _ = ws_server.disconnect(channel_id);

    let _ = session.close(close_reason).await;

    Ok(())
}

async fn process_text_msg(
    db: &Arc<Surreal<Any>>,
    ws_metadata: &WrappedWsData,
    _ws_server: &WsServerHandle,
    session: &mut actix_ws::Session,
    text: &str,
) -> Result<Option<WrappedWsData>, KromerError> {
    // strip leading and trailing whitespace (spaces, newlines, etc.)
    let msg = text.trim();

    //// For testing, consider echoing the message back
    // let result = session.text(msg).await;

    // result.map_err(|_| KromerError::WebSocket(WebSocketError::MessageSend))?;

    let parsed_msg_result: Result<IncomingWebsocketMessage, serde_json::Error> =
        serde_json::from_str(msg);

    let parsed_msg = match parsed_msg_result {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("Serde error: {}", err);
            tracing::info!("Could not parse JSON for UUID: {}", ws_metadata.token);
            return Err(KromerError::WebSocket(WebSocketError::JsonParseRead));
        }
    };

    let msg_type = parsed_msg.message_type;
    let msg_id = parsed_msg.id;

    let _msg_result = match msg_type {
        WebSocketMessageType::Me => route_get_me(msg_id, db, ws_metadata).await,
        _ => {
            // TODO: This is just an example, we should error here with a good error message
            Ok(OutgoingWebSocketMessage {
                ok: Some(false),
                id: msg_id,
                message: WebSocketMessageType::Error {
                    error: ErrorResponse {
                        error: "invalid_parameter".to_string(),
                        message: Some("Invalid parameter type".to_string()),
                    },
                },
            })
        }
    };

    match _msg_result {
        Ok(value) => {
            let _ = session
                .text(serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string()))
                .await;
        }
        Err(_) => {
            let _ = session.text("Error").await;
        }
    }

    Ok(None)
}

async fn spawn_keepalive(ws_server: WsServerHandle, conn: Uuid) -> (JoinHandle<()>, AbortHandle) {
    let (abort_handle, _) = AbortHandle::new_pair();

    let join_handle = tokio::spawn(async move {
        let mut interval = interval(KEEPALIVE_INTERVAL);

        loop {
            interval.tick().await;
            let cur_time = convert_to_iso_string(chrono::offset::Utc::now());
            let keepalive_time = WebSocketMessageType::Keepalive {
                server_time: cur_time.clone(),
            };
            let return_message =
                serde_json::to_string(&keepalive_time).unwrap_or_else(|_| "{}".to_string());
            let _ = ws_server
                .send_message(conn, return_message.to_string())
                .await;
        }
    });

    (join_handle, abort_handle)
}
