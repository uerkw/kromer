use crate::errors::{websocket::WebSocketError, KromerError};
use std::{
    pin::pin,
    str::FromStr,
    time::{Duration, Instant},
};

use super::{
    types::common::WebSocketMessageType, utils::parse_message::parse_message,
    wrapped_ws::WrappedWsData, ws_server::WsServerHandle,
};
use actix_ws::AggregatedMessage;
use futures_util::{
    future::{select, Either},
    stream::AbortHandle,
    StreamExt,
};
use serde_json::json;
use surrealdb::Uuid;
use tokio::{sync::mpsc, task::JoinHandle, time::interval};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// TODO: 30 seconds for debugging, should probably be around 10 seconds for client timeout in prooduction
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);

pub async fn handle_ws(
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
                                &ws_metadata,
                                &ws_server,
                                &mut session,
                                &text,
                                channel_id,
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
    ws_metadata: &WrappedWsData,
    _ws_server: &WsServerHandle,
    session: &mut actix_ws::Session,
    text: &str,
    _conn: Uuid,
) -> Result<Option<WrappedWsData>, KromerError> {
    // strip leading and trailing whitespace (spaces, newlines, etc.)
    let msg = text.trim();

    // Echo the message back for now..
    let result = session.text(msg).await;

    result.map_err(|_| KromerError::WebSocket(WebSocketError::MessageSend))?;

    let msg_as_json = parse_message(msg.to_string());

    let msg_as_json = msg_as_json.map_err(|_| {
        tracing::info!("Could not parse JSON for UUID: {_conn}");
        KromerError::WebSocket(WebSocketError::JsonParseRead)
    })?;

    let msg_id = msg_as_json["id"].as_i64().unwrap_or(0);
    let msg_type = msg_as_json["type"].as_str().unwrap_or("motd");
    tracing::debug!("Message type for {_conn} message ID: {msg_id} was `{msg_type}`");

    let msg_type = WebSocketMessageType::from_str(msg_type).inspect_err(|_| {
        tracing::info!("Could not parse message type for UUID: {_conn}");
    })?;

    // TODO: Testing, extract these out into reusable bits of controller logic...
    match msg_type {
        WebSocketMessageType::Address => {
            let target_address = msg_as_json["address"].as_str();
            let response = json!({"id": msg_id, "ok": "true", "address": target_address});
            let _ = session.text(response.to_string()).await;
        }
        WebSocketMessageType::Login => {}
        WebSocketMessageType::Logout => {
            let new_ws_metadata = WrappedWsData::new(ws_metadata.token, "guest".to_string(), None);
            return Ok(Some(new_ws_metadata));
        }
        WebSocketMessageType::Motd => {
            let response =
                json!({"id": msg_id, "ok":"true", "motd": "This is where the MOTD will go"});
            let _ = session.text(response.to_string()).await;
        }
        WebSocketMessageType::Me => {
            let response = if ws_metadata.is_guest() {
                json!({"ok": true, "isGuest": true, "id": msg_id })
            } else {
                json!({"ok": true, "isGuest": false, "address": ws_metadata.address, "id": msg_id})
            };
            //let response = json!({"id": _msg_id, "ok": "true", "address": ws_metadata.address});
            let _ = session.text(response.to_string()).await;
        }
        _ => {}
    }

    Ok(None)
}

async fn spawn_keepalive(ws_server: WsServerHandle, conn: Uuid) -> (JoinHandle<()>, AbortHandle) {
    let (abort_handle, _) = AbortHandle::new_pair();

    let join_handle = tokio::spawn(async move {
        let mut interval = interval(KEEPALIVE_INTERVAL);

        loop {
            interval.tick().await;
            let cur_time = chrono::offset::Utc::now();
            let return_message = json!({"type":"keepalive", "server_time": cur_time.to_rfc3339()});
            let _ = ws_server
                .send_message(conn, return_message.to_string())
                .await;
        }
    });

    (join_handle, abort_handle)
}
