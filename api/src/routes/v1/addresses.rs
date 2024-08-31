use actix_web::{error, get, web, Error, HttpResponse, Result};

use kromer_economy_entity::addresses;
use kromer_economy_service::Query;
use serde_json::json;

use crate::AppState;

#[derive(Debug, serde::Deserialize)]
struct LimitAndOffset {
    limit: Option<u64>,
    offset: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct ShouldFetchNames {
    should_fetch_names: Option<bool>,
}

#[get("/")]
async fn list_addresses(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();

    let conn = &state.conn;
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let addresses = Query::fetch_addresses(conn, limit, offset)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let total = Query::count_total_addresses(conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let response: Vec<serde_json::Value> = addresses
        .iter()
        .map(|model| {
            json!({
                "address": model.address,
                "balance": model.balance,
                "totalin": model.total_in,
                "totalout": model.total_out,
                "firstseen": model.first_seen,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "count": response.len(),
        "total": total,
        "addresses": response,
    })))
}

#[get("/{address}")]
async fn get_specific_address(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<ShouldFetchNames>,
) -> Result<HttpResponse, Error> {
    let address = path.into_inner(); // TODO: Return if invalid address (may not be possible)
    let query = query.into_inner();

    let conn = &state.conn;
    let should_fetch_names = query.should_fetch_names.unwrap_or(false);

    let addr: Option<addresses::Model> = Query::find_address(conn, &address, should_fetch_names)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // Kinda cursed but it works
    match addr {
        Some(addr) => Ok(HttpResponse::Ok().json(json!({
            "ok": true,
            "address": {
                "address": addr.address,
                "balance": addr.balance,
                "totalin": addr.total_in,
                "totalout": addr.total_out,
                "firstseen": addr.first_seen,
            }
        }))),
        None => Ok(HttpResponse::Ok().json(json!({
                "ok": false,
                "error": "address_not_found"
            }
        ))),
    }
}

#[get("/rich")]
async fn get_richest_addresses(
    state: web::Data<AppState>,
    path: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, Error> {
    let path = path.into_inner();
    let limit = path.limit.unwrap_or(50);
    let offset = path.offset.unwrap_or(0);

    let conn = &state.conn;

    let richest_addresses: Vec<addresses::Model> =
        Query::find_richest_addresses(conn, limit, offset)
            .await
            .map_err(error::ErrorInternalServerError)?;

    let total = Query::count_total_addresses(conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let response: Vec<serde_json::Value> = richest_addresses
        .into_iter()
        .map(|addr| {
            json!({
                "address": addr.address,
                "balance": addr.balance,
                "totalin": addr.total_in,
                "totalout": addr.total_out,
                "firstseen": addr.first_seen,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "count": response.len(),
        "total": total,
        "addresses": response,
    })))
}

// This is missing the `excludeMined` query paramater, we don't have mining.
#[get("/{address}/transactions")]
async fn get_address_transactions(
    state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let address = address.into_inner(); // TODO: Return if invalid address (may not be possible)

    let conn = &state.conn;

    let addr = Query::find_address(conn, &address, false)
        .await
        .map_err(error::ErrorInternalServerError)?;

    if addr.is_none() {
        return Ok(HttpResponse::Ok().json(json!({
            "ok": false,
            "error": "address_not_found"
        })));
    }

    // Im not particularly sure about the function name here
    let transaction_count = Query::count_total_transactions_from_address(conn, &address)
        .await
        .map_err(error::ErrorInternalServerError)?;
    let transactions = Query::find_transactions_from_address(conn, &address)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // TODO: This is missing 2 fields, `metadata` and `type`, type can be `transfer`, `name_purchase`, `name_a_record`, or `name_transfer`. `metadata` is the CommonMeta shit.
    let response: Vec<serde_json::Value> = transactions
        .into_iter()
        .map(|tx| {
            json!({
                "id": tx.id,
                "from": tx.from,
                "to": tx.to,
                "value": tx.value,
                "time": tx.time,
                "name": tx.name,
                "sent_metaname": tx.sent_metaname,
                "sent_name": tx.sent_name,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "count": response.len(),
        "total": transaction_count,
        "transactions": response,
    })))
}

#[get("/{address}/names")]
async fn get_address_names(
    _state: web::Data<AppState>,
    address: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let address = address.into_inner();

    Ok(HttpResponse::Ok().body(address))
}
