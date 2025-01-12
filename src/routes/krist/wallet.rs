use actix_web::{get, web, HttpResponse};

use crate::database::models::wallet::Model as Wallet;
use crate::errors::{wallet::WalletError, KromerError};
use crate::models::addresses::AddressJson;
use crate::{routes::PaginationParams, AppState};

#[get("/")]
async fn wallet_list(
    state: web::Data<AppState>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, KromerError> {
    let pagination = pagination.into_inner();
    let db = &state.db;

    let wallets = Wallet::all(db, &pagination).await?;
    let response: Vec<AddressJson> = wallets.into_iter().map(|wallet| wallet.into()).collect();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{address}")]
async fn wallet_get(
    state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let address = address.into_inner();
    let db = &state.db;

    let wallet = Wallet::get_by_address_excl(db, address).await?;

    match wallet {
        Some(wallet) => {
            let wallet: AddressJson = wallet.into();
            Ok(HttpResponse::Ok().json(wallet))
        }
        None => Err(KromerError::Wallet(WalletError::NotFound)),
    }
}

#[get("/richest")]
async fn wallet_richest(
    state: web::Data<AppState>,
    pagination: web::Query<PaginationParams>,
) -> Result<HttpResponse, KromerError> {
    let pagination = pagination.into_inner();
    let db = &state.db;

    let wallets = Wallet::get_richest(db, &pagination).await?;
    let response: Vec<AddressJson> = wallets.into_iter().map(|wallet| wallet.into()).collect();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{address}/transactions")]
async fn wallet_get_transactions(
    _state: web::Data<AppState>,
    _address: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    // let address = address.into_inner();
    // let db = &state.db;

    // let wallet = Wallet::get_by_address_excl(db, address).await?;

    todo!("Not yet implemented, new method required on transaction model")
}

#[get("/{address}/names")]
async fn wallet_get_names(
    _state: web::Data<AppState>,
    _address: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    // let address = address.into_inner();
    // let db = &state.db;

    // let wallet = Wallet::get_by_address_excl(db, address).await?;

    todo!("Not yet implemented, unsure how to approach")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/addresses")
            .service(wallet_list)
            .service(wallet_richest)
            .service(wallet_get)
            .service(wallet_get_transactions)
            .service(wallet_get_names),
    );
}
