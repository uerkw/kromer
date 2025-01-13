use std::sync::Arc;

use surrealdb::{engine::any::Any, Surreal};
use tokio::sync::Mutex;
use websockets::{token_cache::TokenCache, ws_manager::WsDataManager, ws_server::WsServerHandle};

use sea_orm::DatabaseConnection;

pub mod database;
pub mod pg_errors;
pub mod pg_responses;
pub mod pg_controllers;
pub mod pg_utils;
pub mod errors;
pub mod guards;
pub mod entities;
pub mod models;
pub mod routes;
pub mod websockets;

#[derive(Debug)]
pub struct AppState {
    pub pg_db: DatabaseConnection,
    pub db: Arc<Surreal<Any>>,
    pub ws_server_handle: WsServerHandle,
    pub name_cost: f32,
    pub token_cache: Arc<Mutex<TokenCache>>,
    pub ws_manager: Arc<Mutex<WsDataManager>>,
}
