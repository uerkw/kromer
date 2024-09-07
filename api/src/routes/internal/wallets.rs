use actix_web::{post, web, HttpResponse};

use crate::util::guards::admin_key_guard;
use crate::{errors::KromerError, AppState};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_wallets);
}

#[post("/create", guard = "admin_key_guard")]
pub async fn create_wallets(_state: web::Data<AppState>) -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}
