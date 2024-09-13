use addresses::Address;
use serde::{Deserialize, Serialize};

pub mod addresses;
pub mod generic;
pub mod names;
pub mod transactions;
pub mod websocket;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub ok: bool,
    pub authed: bool,
    pub address: String,
}