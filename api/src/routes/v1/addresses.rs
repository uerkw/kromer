use actix_web::{get, web, HttpResponse, Result};

use kromer_economy_entity::addresses;
use kromer_economy_service::controller::{
    AddressController, NameController, TransactionController,
};
use serde_json::json;

use crate::errors::{AddressError, KromerError};
use crate::{routes::LimitAndOffset, AppState};

use crate::responses::v1::addresses::{Address, AddressResponse};
use crate::responses::v1::names::{Name, NameResponse};
use crate::responses::v1::transactions::{Transaction, TransactionResponse};

#[derive(Debug, serde::Deserialize)]
struct ShouldFetchNames {
    should_fetch_names: Option<bool>,
}

#[get("")]
async fn list_addresses(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, KromerError> {
    let query = query.into_inner();

    let conn = &state.conn;
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let addresses = AddressController::addresses(conn, limit, offset)
        .await
        .map_err(KromerError::Database)?;
    let total = AddressController::count(conn)
        .await
        .map_err(KromerError::Database)?;

    let addrs: Vec<Address> = addresses
        .iter()
        .map(|model| Address {
            address: model.address.clone(), // NOTE(sov): I really do not want to clone here, should be refactored.
            balance: model.balance,
            total_in: model.total_in,
            total_out: model.total_out,
            first_seen: model.first_seen,
        })
        .collect();

    let response = AddressResponse {
        ok: true,
        total,
        count: addresses.len() as u64,
        addresses: addrs,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{address}")]
async fn get_specific_address(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<ShouldFetchNames>,
) -> Result<HttpResponse, KromerError> {
    let address = path.into_inner(); // TODO: Return if invalid address (may not be possible)
    let query = query.into_inner();

    let conn = &state.conn;
    let should_fetch_names = query.should_fetch_names.unwrap_or(false);

    let addr = AddressController::fetch_address(conn, &address, should_fetch_names)
        .await
        .map_err(KromerError::Database)?;

    // Kinda cursed but it works
    match addr {
        Some(addr) => {
            let address = Address {
                address,
                balance: addr.balance,
                total_in: addr.total_in,
                total_out: addr.total_out,
                first_seen: addr.first_seen,
            };

            // TODO: Make the `count` and `total` fields optional and skip them if they're `None`
            Ok(HttpResponse::Ok().json(json!({
                "ok": true,
                "address": address
            })))
        }
        None => Err(KromerError::Address(AddressError::NotFound(address))),
    }
}

#[get("/rich")]
async fn get_richest_addresses(
    state: web::Data<AppState>,
    path: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, KromerError> {
    let path = path.into_inner();
    let limit = path.limit.unwrap_or(50);
    let offset = path.offset.unwrap_or(0);

    let conn = &state.conn;

    let richest_addresses: Vec<addresses::Model> = AddressController::richest(conn, limit, offset)
        .await
        .map_err(KromerError::Database)?;

    let total = AddressController::count(conn)
        .await
        .map_err(KromerError::Database)?;

    let addresses: Vec<Address> = richest_addresses
        .into_iter()
        .map(|addr| Address {
            address: addr.address,
            balance: addr.balance,
            total_in: addr.total_in,
            total_out: addr.total_out,
            first_seen: addr.first_seen,
        })
        .collect();

    let response = AddressResponse {
        ok: true,
        total,
        count: addresses.len() as u64,
        addresses,
    };

    Ok(HttpResponse::Ok().json(response))
}

// This is missing the `excludeMined` query paramater, we don't have mining.
#[get("/{address}/transactions")]
async fn get_address_transactions(
    state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let address = address.into_inner(); // TODO: Return if invalid address (may not be possible)

    let conn = &state.conn;

    let addr = AddressController::fetch_address(conn, &address, false)
        .await
        .map_err(KromerError::Database)?;

    if addr.is_none() {
        return Err(KromerError::Address(AddressError::NotFound(address)));
    }

    // Im not particularly sure about the function name here
    let transactions = AddressController::transactions(conn, &address)
        .await
        .map_err(KromerError::Database)?;
    let transaction_count = TransactionController::count(conn)
        .await
        .map_err(KromerError::Database)?;

    // TODO: This is missing the field `type`, type can be `transfer`, `name_purchase`, `name_a_record`, or `name_transfer`.
    let response: Vec<Transaction> = transactions
        .into_iter()
        .map(|tx| Transaction {
            id: tx.id,
            from: tx.from,
            to: tx.to,
            value: tx.value,
            time: tx.time,
            name: tx.name,
            sent_metaname: tx.sent_metaname,
            sent_name: tx.sent_name,
            metadata: tx.metadata,
        })
        .collect();

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
) -> Result<HttpResponse, KromerError> {
    let address = address.into_inner();

    let path = path.into_inner();
    let limit = path.limit.unwrap_or(50);
    let offset = path.offset.unwrap_or(0);

    let conn = &state.conn;

    let addr = AddressController::fetch_address(conn, &address, false)
        .await
        .map_err(KromerError::Database)?;

    if addr.is_none() {
        return Err(KromerError::Address(AddressError::NotFound(address)));
    }

    let names_count = NameController::names_owned_by_address(conn, &address)
        .await
        .map_err(KromerError::Database)?;

    let names = AddressController::names(conn, &address, limit, offset)
        .await
        .map_err(KromerError::Database)?;

    let response: Vec<Name> = names
        .into_iter()
        .map(|name| Name {
            name: name.name,
            owner: name.owner,
            registered: name.registered,
            updated: name.updated,
            metadata: name.metadata,
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "count": response.len(),
        "total": names_count,
        "names": response,
    })))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_addresses);
    cfg.service(get_specific_address);
    cfg.service(get_address_transactions);
    cfg.service(get_address_names);
    cfg.service(get_richest_addresses);
}
