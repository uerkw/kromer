use std::str::FromStr;

use actix::Actor;
use actix_web::{
    post, route, web::{self, Data}, HttpRequest, HttpResponse, Responder
};

use actix_ws::{CloseCode, CloseReason, Message};
use serde_json::json;
use surrealdb::Uuid;
use tokio::time::{sleep, Duration};

use crate::{
    database::models::wallet::Model as Wallet, errors::{wallet::WalletError, websocket::WebSocketError, KromerError}, ws::{
        actors::session::WebSocketSession,
        types::{
            actor_message::{
                AddToken, CheckTokenExists, CloseWebSocket, GetActiveSessions, ReceiveMessage, RemoveToken, SetCacheConnection
            }, server::TokenParams, session::KromerAddress
        }, utils,
    }, AppState
};

#[derive(serde::Deserialize)]
pub struct WebSocketInitData {
    privatekey: Option<String>,
}

#[post("/start")]
pub async fn start_ws(
    state: Data<AppState>,
    details: web::Json<WebSocketInitData>
)-> Result<HttpResponse, KromerError>{
    let db = &state.db;
    let ws_manager = (&state.new_ws_manager).clone();

    let ws_privatekey = &details.privatekey;
    let ws_privatekey_2 = ws_privatekey.clone();

    let mut address = Some(String::from("guest"));

    if let Some(privatekey) = ws_privatekey {
        // This should error back in the request if the wallet key is invalid.
        let wallet = Wallet::verify(db, privatekey.to_string())
            .await
            .map_err(KromerError::Database)?
            .ok_or_else(|| KromerError::Wallet(WalletError::InvalidPassword))?;
        
        address = Some(wallet.address);
    } 
    let address = KromerAddress::from_string(address.unwrap_or("guest".to_string()));

    let new_uuid = Uuid::new_v4();

    // Construct a message to the WebSocketManagerActor to add the uuid.
    // Construct the token parameters
    let token_params = TokenParams{ address: address, privatekey: ws_privatekey_2.unwrap_or("".to_string()).to_string()};
    // Construct the actual message
    let add_token_request = AddToken(new_uuid, token_params ); 
    // Send it over the manager actor
    let _ = ws_manager.send(add_token_request).await;

    // We also need to spawn a thread for deleting this session...
    actix_web::rt::spawn( async move {
        sleep(Duration::from_secs(30)).await;
        
        let _ = ws_manager.send(RemoveToken(new_uuid)).await;
    });

    // Finally, construct a url for the user
    let return_url = utils::make_url::make_url(new_uuid)?;

    Ok(HttpResponse::Ok().json(json!({
        "url": return_url
    })))

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

    let uuid = Uuid::from_str(&token).map_err(|_| KromerError::WebSocket(WebSocketError::UuidNotFound))?;
    // Look up from the Token map to see if this Uuid currently exists or not
    // Construct a request message
    let token_lookup_request = CheckTokenExists(uuid);
    let lookup_result = _ws_manager.send(token_lookup_request).await;

    let mut lookup_exists = false;
    match lookup_result {
        Ok(value) => {
            if value {
                lookup_exists= true;
            }
        }
        Err(_) => {
            return Err(KromerError::WebSocket(WebSocketError::UuidNotFound));
        }
    }

    if !lookup_exists {
        return Err(KromerError::WebSocket(WebSocketError::UuidNotFound));
    }

    // We handled the lookup case, now we can safely consume the UUID

    let token_remove_request = RemoveToken(uuid);
    let _ = _ws_manager.send(token_remove_request).await;     


    let (response, mut _session, mut _msg_stream) =
        actix_ws::handle(&req, body).or_else(|_| Err(WebSocketError::RoomCreation))?;

    let address = Some(KromerAddress::from_string("guest".to_string()));
    let token_uuid = Uuid::from_str(&token).or_else(|_| Err(KromerError::WebSocket(WebSocketError::InvalidUuid)))?;

    let wrapped_ws_session = WebSocketSession::new(
        token_uuid,
        address,
        Some("privatekey".to_string()),
        _session,
        _ws_manager.clone(),
    );

    let ws_actor_addr = wrapped_ws_session.start();
    let cloned_ws_actor_addr = ws_actor_addr.clone();
    let _ = _ws_manager
        .send(SetCacheConnection(token_uuid, ws_actor_addr))
        .await;

    let thread_ws_manager = _ws_manager.clone();
    let thread_token_uuid = token_uuid.clone();

    // Receive thread
    actix_web::rt::spawn(async move {
        // This here is debug related, and doesn't necessarily have to make it into the final impl
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
                        "[SPAWNED_WS_THREAD][RECEIVE] Client WS Closed with Code: {:?}, Description: {:?}",
                        close_reason.code,
                        close_reason.description
                    );
                    break;
                }
                Message::Text(msg) => {
                    tracing::debug!("[SPAWNED_WS_THREAD] Received text, message: {msg}");
                    let to_server_msg = ReceiveMessage(thread_token_uuid, msg.to_string());
                    let _ = thread_ws_manager.send(to_server_msg).await;
                }
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
        "[SPAWNED_WS_THREAD] Active Sessions On Open: {:?}",
        active_sessions
    );
    Ok(response)
}
