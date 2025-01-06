use crate::errors::{websocket::WebSocketError, KromerError};
use std::{
    pin::pin,
    time::{Duration, Instant},
};

use super::ws_server::WsServerHandle;
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
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);

pub async fn handle_ws(
    ws_server: WsServerHandle,
    mut session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
) -> Result<(), KromerError> {
    let debug_span = tracing::span!(tracing::Level::DEBUG, "WS_HANDLER");
    let _tracing_debug_enter = debug_span.enter();

    let mut name: Option<String> = None;
    let mut last_heartbeat = Instant::now();
    let mut heartbeat_interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    let conn_id = match ws_server.connect(conn_tx).await {
        Ok(value) => value,
        Err(_) => return Err(KromerError::WebSocket(WebSocketError::HandshakeError)),
    };

    let msg_stream = msg_stream
        .max_frame_size(64 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let (_keepalive_join_handle, keepalive_abort_handle) =
        spawn_keepalive(ws_server.clone(), conn_id).await;

    let mut msg_stream = pin!(msg_stream);

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
                            let error_msg = json!({"ok": "false", "error": "Message larger than 512 characters"}).to_string();
                            tracing::info!("Message received was larger than 512 characters");
                            let _ = session.text(error_msg).await;
                        } else {
                            tracing::info!("Message received: {text}");
                            let _ = process_text_msg(
                                &ws_server,
                                &mut session,
                                &text,
                                conn_id,
                                &mut name,
                            )
                            .await;
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

    let _ = ws_server.disconnect(conn_id);

    let _ = session.close(close_reason).await;

    Ok(())
}

async fn process_text_msg(
    _ws_server: &WsServerHandle,
    session: &mut actix_ws::Session,
    text: &str,
    _conn: Uuid,
    _name: &mut Option<String>,
) -> Result<(), KromerError> {
    // strip leading and trailing whitespace (spaces, newlines, etc.)
    let msg = text.trim();

    // Echo the message back for now..
    let result = session.text(msg).await;

    result.map_err(|_| KromerError::WebSocket(WebSocketError::MessageSend))?;

    Ok(())
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
