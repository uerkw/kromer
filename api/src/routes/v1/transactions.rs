use actix_web::{get, post, web, HttpResponse, Result};

use crate::{errors::KromerError, routes::LimitAndOffset, AppState};

// https://krist.dev/docs/#api-TransactionGroup-GetTransactions
#[get("")]
async fn list_transactions(state: web::Data<AppState>) -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

// https://krist.dev/docs/#api-TransactionGroup-GetLatestTransactions
#[get("/latest")]
async fn list_latest_transactions(state: web::Data<AppState>, _query: web::Query<LimitAndOffset>) -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

// https://krist.dev/docs/#api-TransactionGroup-GetSpecificTransaction
#[get("/{transaction_id}")]
async fn get_specific_transaction(state: web::Data<AppState>, _path: web::Path<u64>) -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[post("/")]
async fn make_transaction() -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_transactions);
    cfg.service(list_latest_transactions);
    cfg.service(get_specific_transaction);
    cfg.service(make_transaction);
}