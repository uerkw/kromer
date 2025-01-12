use std::sync::Arc;

use actix::Addr;
use surrealdb::{engine::any::Any, Surreal};
use tokio::sync::Mutex;
use websockets::{token_cache::TokenCache, ws_manager::WsDataManager, ws_server::WsServerHandle};
use ws::actors::server::WsServerActor as OldWebSocketServer;

pub mod database;
pub mod errors;
pub mod guards;
pub mod models;
pub mod routes;
pub mod websockets;
pub mod ws;

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<Surreal<Any>>,
    pub old_ws_manager: Addr<OldWebSocketServer>,
    pub ws_server_handle: WsServerHandle,
    pub token_cache: Arc<Mutex<TokenCache>>,
    pub ws_manager: Arc<Mutex<WsDataManager>>,
}
