use actix_web::{post, web, HttpResponse};
use serde_json::json;
use surrealdb::Uuid;

use std::env;

use crate::{
    database::models::wallet::Model as Wallet,
    errors::{wallet::WalletError, websocket::WebSocketError, KromerError},
    websockets::{
        message::{CreateRoom, SetCacheConnection},
        session::KromerWsSession,
    },
    AppState,
};

#[derive(serde::Deserialize)]
pub struct WebSocketInitData {
    privatekey: Option<String>,
    name: Option<String>,
}

#[post("/start")]
async fn request_token(
    state: web::Data<AppState>,
    details: web::Json<WebSocketInitData>,
) -> Result<HttpResponse, KromerError> {
    // Grab our app state
    let db = &state.db;
    let ws_manager = (&state.ws_manager).clone();

    let ws_privatekey = &details.privatekey;
    let ws_name = &details.name;

    let create_room_request = CreateRoom(Some("test".to_string()));
    let room_name_result = ws_manager.send(create_room_request).await;

    let room_name_msg: String;

    match room_name_result {
        Ok(room_name) => {
            room_name_msg = room_name;
        }
        Err(mailbox_error) => {
            tracing::error!("Error creating room: {:?}", mailbox_error);
            return Err(KromerError::WebSocket(WebSocketError::RoomCreation));
        }
    }

    let new_uuid = Uuid::new_v4();
    let schema = "ws";
    let host =
        env::var("HOST").map_err(|_| KromerError::WebSocket(WebSocketError::ServerConfigError))?;
    let port =
        env::var("PORT").map_err(|_| KromerError::WebSocket(WebSocketError::ServerConfigError))?;

    let server_url = format!("{host}:{port}");
    let full_url = format!("{schema}://{server_url}/ws/gateway/{new_uuid}");
    let mut address = Some(String::from("guest"));

    if let Some(privatekey) = ws_privatekey {
        // Verify the wallet address
        let wallet = Wallet::verify(db, privatekey.to_string())
            .await
            .map_err(KromerError::Database)?
            .ok_or_else(|| KromerError::Wallet(WalletError::InvalidPassword))?;

        address = Some(wallet.address);
    }

    println!("New UUID was: {new_uuid}");

    let session = KromerWsSession::new(
        new_uuid,
        room_name_msg,
        ws_manager.clone(),
        address.clone(),
        ws_privatekey.clone(),
        ws_name.clone(),
    );
    // Construct the message and send it to be cached
    let conn_to_cache = session;
    let conn_cache_request = SetCacheConnection(new_uuid, conn_to_cache);
    let msg_result = ws_manager.send(conn_cache_request).await;

    match msg_result {
        Ok(response) => {
            tracing::debug!("Successfully sent message to actor: {:?}", response);
        }
        Err(mailbox_error) => {
            tracing::error!("Failed to send message to actor: {:?}", mailbox_error);
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "url": full_url,
    })))
}
