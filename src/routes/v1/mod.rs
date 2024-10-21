mod name;
mod transaction;
mod wallet;

use actix_web::{get, web, HttpResponse};

use crate::errors::KromerError;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginDetail {
    pub password: String,
}

#[get("/")]
async fn index_get() -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}

#[get("/version")]
async fn version_get() -> Result<HttpResponse, KromerError> {
    let response = serde_json::json!({
        "version": VERSION,
    });

    Ok(HttpResponse::Ok().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index_get);
    cfg.service(version_get);
    cfg.configure(wallet::config);
    cfg.configure(transaction::config);
    cfg.configure(name::config);
}
