use surrealdb::{engine::any::Any, Surreal};

pub mod database;
pub mod errors;
pub mod guards;
pub mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
}
