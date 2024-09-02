use actix_web::{get, post, web, HttpResponse};
use kromer_economy_service::controller::{AddressController, NameController, NameRegistration};
use serde_json::json;

use crate::errors::{AddressError, KromerError, NameError};
use crate::{routes::LimitAndOffset, AppState};

use crate::responses::v1::names::{Name, NameResponse};

#[derive(serde::Deserialize)]
struct RegisterNameRequest {
    #[serde(rename = "privatekey")]
    private_key: String,
}

// https://krist.dev/docs/#api-NameGroup-GetNames
#[get("")]
async fn list_names(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, KromerError> {
    let query = query.into_inner();

    let conn = &state.conn;

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let names = NameController::list_names(conn, limit, offset)
        .await
        .map_err(KromerError::Database)?;

    let total = NameController::name_count(conn)
        .await
        .map_err(KromerError::Database)?;

    let response: Vec<serde_json::Value> = names
        .into_iter()
        .map(|name| {
            json!({
                "name": name.name,
                "owner": name.owner,
                "original_owner": name.original_owner,
                "registered": name.registered,
                "updated": name.updated,
                "transferred": name.transferred,
                "a": name.metadata,
                "unpaid": name.unpaid,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "count": response.len(),
        "total": total,
        "names": response,
    })))
}

// https://krist.dev/docs/#api-NameGroup-CheckName
#[get("/check/{name}")]
async fn check_name_availability(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let name = path.into_inner();

    let conn = &state.conn;

    let is_available = NameController::is_name_available(conn, &name)
        .await
        .map_err(KromerError::Database)?;

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "available": is_available,
    })))
}

// https://krist.dev/docs/#api-NameGroup-GetName
#[get("/{name}")]
async fn get_specific_name(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let name = path.into_inner();

    let conn = &state.conn;

    match NameController::get_name(conn, &name).await {
        Ok(Some(name)) => Ok(HttpResponse::Ok().json(json!({
            "ok": true,
            "name": {
                "name": name.name,
                "owner": name.owner,
                "original_owner": name.original_owner,
                "registered": name.registered,
                "updated": name.updated,
                "transferred": name.transferred,
                "a": name.metadata,
                "unpaid": name.unpaid,
            }
        }))),
        Ok(None) => Err(KromerError::Name(NameError::NameNotFound(name))),
        Err(e) => Err(KromerError::Database(e)),
    }
}

// https://krist.dev/docs/#api-NameGroup-RegisterName
#[post("/{name}")]
async fn register_name(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<RegisterNameRequest>,
) -> Result<HttpResponse, KromerError> {
    let name = path.into_inner();

    let conn = &state.conn;

    // TODO: Implement proper authentication and address get from private key
    let _private_key = &body.private_key;
    let owner_address = "TODO_GET_ADDRESS_FROM_PRIVATE_KEY"; // TODO: Get address from private key, should return address model.

    let name_available = NameController::is_name_available(conn, &name)
        .await
        .map_err(KromerError::Database)?;

    if !name_available {
        return Err(KromerError::Name(NameError::NameTaken(name)));
    }

    let owner = AddressController::fetch_address(conn, owner_address, false)
        .await
        .map_err(KromerError::Database)?
        .ok_or_else(|| KromerError::Address(AddressError::NotFound(owner_address.to_string())))?;

    // TODO: Check if the user has enough balance to register the name
    // if owner.balance < state.name_cost {
    //     return Ok(HttpResponse::Ok().json(json!({
    //         "ok": false,
    //         "error": "insufficient_balance"
    //     })));
    // }

    let registration = NameRegistration { name, owner };

    match NameController::register_name(conn, registration).await {
        Ok(_registered_name) => {
            // TODO: Deduct the name cost from the user's balance
            // AddressController::deduct_balance(conn, owner_address, state.name_cost).await?;

            Ok(HttpResponse::Ok().json(json!({
                "ok": true,
            })))
        }
        Err(e) => Err(KromerError::Database(e)),
    }
}

// https://krist.dev/docs/#api-NameGroup-GetNameCost
#[get("/cost")]
async fn get_cost_of_name(state: web::Data<AppState>) -> Result<HttpResponse, KromerError> {
    let name_cost = state.name_cost;

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "name_cost": name_cost,
    })))
}

// https://krist.dev/docs/#api-NameGroup-GetNewNames
#[get("/new")]
async fn get_newest_names(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, KromerError> {
    let query = query.into_inner();

    let conn = &state.conn;

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let names = NameController::get_newest_names(conn, limit, offset)
        .await
        .map_err(KromerError::Database)?;

    let total = NameController::name_count(conn)
        .await
        .map_err(KromerError::Database)?;

    let response: Vec<serde_json::Value> = names
        .into_iter()
        .map(|name| {
            json!({
                "name": name.name,
                "owner": name.owner,
                "original_owner": name.original_owner,
                "registered": name.registered,
                "updated": name.updated,
                "transferred": name.transferred,
                "a": name.metadata,
                "unpaid": name.unpaid,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "ok": true,
        "count": response.len(),
        "total": total,
        "names": response,
    })))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_names);
    cfg.service(get_specific_name);
    cfg.service(register_name);
    cfg.service(get_cost_of_name);
    cfg.service(get_newest_names);
    cfg.service(check_name_availability);
}
