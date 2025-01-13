use actix_web::{get, web, HttpResponse, Result};

use crate::pg_controllers::address::AddressController;
use crate::pg_controllers::name::NameController;
use crate::pg_controllers::transaction::TransactionController;
use crate::entities::address;
use crate::pg_errors::address::AddressError;
use crate::AppState;

use crate::pg_responses::v1::address::{Address, AddressResponse, SingularAddressResponse};
use crate::pg_responses::v1::name::{Name, NameResponse};
use crate::pg_responses::v1::transaction::{Transaction, TransactionResponse};
use crate::pg_errors::PgKromerError;
use crate::routes::pg_v1::lib::LimitAndOffset;

#[derive(Debug, serde::Deserialize)]
struct ShouldFetchNames {
    should_fetch_names: Option<bool>,
}

#[get("")]
async fn list_addresses(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, PgKromerError> {
    let query = query.into_inner();

    let conn = &state.pg_db;
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let addresses = AddressController::addresses(conn, limit, offset)
        .await
        .map_err(PgKromerError::Database)?;

    let total = AddressController::count(conn)
        .await
        .map_err(PgKromerError::Database)?;

    let addrs: Vec<Address> = addresses
        .iter()
        .map(Into::into)
        .collect();

    let response = AddressResponse {
        ok: true,
        total,
        count: addresses.len() as u64,
        addresses: addrs,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{addresses}")]
async fn get_specific_address(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<ShouldFetchNames>,
) -> Result<HttpResponse, PgKromerError> {
    let address = path.into_inner();
    let conn = &state.pg_db;
    let should_fetch_names = query.should_fetch_names.unwrap_or(false);

    let addr = AddressController::fetch_address(conn, &address, should_fetch_names)
        .await
        .map_err(PgKromerError::Database)?;

    match addr {
        Some(addr) => {
            let address = addr.into();

            Ok(HttpResponse::Ok().json(SingularAddressResponse { ok: true, address }))
        }
        None => Err(PgKromerError::Address(AddressError::NotFound(address))),
    }
}

#[get("/rich")]
async fn get_richest_addresses(
    state: web::Data<AppState>,
    path: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, PgKromerError> {
    let path = path.into_inner();
    let limit = path.limit.unwrap_or(50);
    let offset = path.offset.unwrap_or(0);

    let conn = &state.pg_db;

    let richest_addresses: Vec<address::Model> = AddressController::richest(conn, limit, offset)
        .await
        .map_err(PgKromerError::Database)?;

    let total = AddressController::count(conn)
        .await
        .map_err(PgKromerError::Database)?;
    
    let addresses: Vec<Address> = richest_addresses.into_iter().map(Into::into).collect();

    let response = AddressResponse {
        ok: true,
        total,
        count: addresses.len() as u64,
        addresses,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{address}/transactions")]
async fn get_address_transactions(
    state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, PgKromerError> {
    let address = address.into_inner(); 

    let conn = &state.pg_db;

    let addr = AddressController::fetch_address(conn, &address, false)
        .await
        .map_err(PgKromerError::Database)?;

    if addr.is_none() {
        return Err(PgKromerError::Address(AddressError::NotFound(address)));
    }

    // Im not particularly sure about the function name here
    let transactions = AddressController::transactions(conn, &address)
        .await
        .map_err(PgKromerError::Database)?;
    let transaction_count = TransactionController::total(conn)
        .await
        .map_err(PgKromerError::Database)?;

    let response: Vec<Transaction> = transactions.into_iter().map(Into::into).collect();

    let response = TransactionResponse {
        ok: true,
        total: transaction_count,
        count: response.len() as u64,
        transactions: response,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{address}/names")]
async fn get_address_names(
    state: web::Data<AppState>,
    address: web::Path<String>,
    path: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, PgKromerError> {
    let address = address.into_inner();

    let path = path.into_inner();
    let limit = path.limit.unwrap_or(50);
    let offset = path.offset.unwrap_or(0);

    let conn = &state.pg_db;

    let addr = AddressController::fetch_address(conn, &address, false)
        .await
        .map_err(PgKromerError::Database)?;

    if addr.is_none() {
        return Err(PgKromerError::Address(AddressError::NotFound(address)));
    }

    let names_count = NameController::names_owned_by_address(conn, &address)
        .await
        .map_err(PgKromerError::Database)?;

    let names = AddressController::names(conn, &address, limit, offset)
        .await
        .map_err(PgKromerError::Database)?;

    let response: Vec<Name> = names.into_iter().map(Into::into).collect();

    let response = NameResponse {
        ok: true,
        total: names_count,
        count: response.len() as u64,
        names: response,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_addresses);
    cfg.service(get_richest_addresses);
    cfg.service(get_specific_address);
    cfg.service(get_address_transactions);
    cfg.service(get_address_names);
}