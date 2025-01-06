use std::sync::{Arc, Mutex};

use actix::Addr;
use surrealdb::{engine::any::Any, Surreal};
use websockets::{token_cache::TokenCache, ws_server::WsServerHandle};
use ws::actors::server::WsServerActor as OldWebSocketServer;

pub mod database;
pub mod errors;
pub mod guards;
pub mod routes;
pub mod websockets;
pub mod ws;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<Surreal<Any>>,
    pub old_ws_manager: Addr<OldWebSocketServer>,
    pub ws_server_handle: WsServerHandle,
    pub token_cache: Arc<Mutex<TokenCache>>,
}
