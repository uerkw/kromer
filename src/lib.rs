use actix::Addr;
use surrealdb::{engine::any::Any, Surreal};
use websockets::server::WebSocketServer;

pub mod database;
pub mod errors;
pub mod guards;
pub mod routes;
pub mod websockets;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
    pub ws_manager: Addr<WebSocketServer>,
}
