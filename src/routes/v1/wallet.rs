use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

use crate::database::models::wallet::Model as Wallet;
use crate::errors::wallet::WalletError;
use crate::errors::KromerError;
use crate::routes::PaginationParams;
use crate::AppState;

use crate::routes::v1::LoginDetail;

#[post("/verify")]
async fn wallet_verify(
    state: web::Data<AppState>,
    detail: web::Json<LoginDetail>,
) -> Result<HttpResponse, KromerError> {
    let detail = detail.into_inner();
    let db = &state.db;

    let wallet = Wallet::verify(db, detail.password)
        .await?
        .ok_or_else(|| KromerError::Wallet(WalletError::InvalidPassword))?;

    Ok(HttpResponse::Ok().json(json!({
        "address": wallet.address
    })))
}

#[get("/list")]
async fn wallet_list(
    state: web::Data<AppState>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, KromerError> {
    let pagination = pagination.into_inner();
    let db = &state.db;

    let wallets = Wallet::all(db, &pagination).await?;

    Ok(HttpResponse::Ok().json(wallets))
}

#[get("/richest")]
async fn wallet_richest(
    state: web::Data<AppState>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, KromerError> {
    let pagination = pagination.into_inner();
    let db = &state.db;

    let wallets = Wallet::get_richest(db, &pagination).await?;

    Ok(HttpResponse::Ok().json(wallets))
}

#[get("/{address}")]
async fn wallet_get(
    state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let address = address.into_inner();
    let db = &state.db;

    let wallet = Wallet::get_by_address_excl(db, address).await?;

    Ok(HttpResponse::Ok().json(wallet))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/wallet")
            .service(wallet_verify)
            .service(wallet_list)
            .service(wallet_richest)
            .service(wallet_get),
    );
}
