mod name;
mod transaction;
mod wallet;

use actix_web::{get, web, HttpResponse};

use crate::errors::KromerError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginDetail {
    pub password: String,
}

#[get("/")]
async fn index_get() -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index_get);
    cfg.configure(wallet::config);
    cfg.configure(transaction::config);
    cfg.configure(name::config);
}
