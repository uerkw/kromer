use std::str::FromStr;

use actix::Actor;
use actix_web::{
    route,
    web::{self, Data},
    HttpRequest, Responder,
};

use actix_ws::Message;
use surrealdb::Uuid;

use crate::{
    errors::{websocket::WebSocketError, KromerError}, ws::{session::WebSocketSession, types::{actor_message::{CloseWebSocket, SetCacheConnection}, session::KromerAddress}}, AppState
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
        .or_else(|_| Err(WebSocketError::RoomCreation))
        .unwrap();

    let address = Some(KromerAddress::from_string("guest".to_string()));
    let token_uuid = Uuid::from_str(&token).or_else(|_| Err(WebSocketError::UuidNotFound)).unwrap();

    let wrapped_ws_session = WebSocketSession::new(token_uuid, address, Some("privatekey".to_string()), _session);

    let ws_actor_addr = wrapped_ws_session.start();
    let cloned_ws_actor_addr = ws_actor_addr.clone();


    let future = async move {
        let _ = _ws_manager.send(SetCacheConnection(token_uuid, ws_actor_addr)).await;
    };

    actix::spawn(future);

    //let cached_conn = _ws_manager.send(GetCacheConnection(token_uuid)).await.or_else(|_| Err(WebSocketError::RoomCreation)).unwrap().unwrap();


    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = _msg_stream.recv().await {
            match msg {
                Message::Close(_msg) => {
                    break;
                }
                Message::Text(msg) => tracing::debug!("Got text: {msg}"),
                _ => break,
            }
        }

        let actor_close_message = CloseWebSocket;
        let _ = cloned_ws_actor_addr.send(actor_close_message).await;
    });

    Ok(response)
}
