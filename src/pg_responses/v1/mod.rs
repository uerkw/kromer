use serde::{Deserialize, Serialize};

pub mod address;
pub mod generic;
pub mod name;
pub mod transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub ok: bool,
    pub authed: bool,
    pub address: String,
}