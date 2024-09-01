use actix_web::{error, get, post, web, Error, HttpResponse};
use kromer_economy_service::controller::{AddressController, NameController, NameRegistration};
use serde_json::json;

use crate::{routes::LimitAndOffset, AppState};

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
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();

    let conn = &state.conn;

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let names = NameController::list_names(conn, limit, offset)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let total = NameController::name_count(conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

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
) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    let conn = &state.conn;

    let is_available = NameController::is_name_available(conn, &name)
        .await
        .map_err(error::ErrorInternalServerError)?;

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
) -> Result<HttpResponse, Error> {
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
        Ok(None) => Ok(HttpResponse::Ok().json(json!({
            "ok": false,
            "error": "name_not_found"
        }))),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

// https://krist.dev/docs/#api-NameGroup-RegisterName
#[post("/{name}")]
async fn register_name(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<RegisterNameRequest>,
) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    let conn = &state.conn;

    // TODO: Implement proper authentication and address get from private key
    let _private_key = &body.private_key;
    let owner_address = "TODO_GET_ADDRESS_FROM_PRIVATE_KEY"; // TODO: Get address from private key, should return address model.

    let name_available = NameController::is_name_available(conn, &name)
        .await
        .map_err(|e| {
            error::ErrorInternalServerError(format!("Failed to check name availability: {}", e))
        })?;

    if !name_available {
        return Ok(HttpResponse::Ok().json(json!({
            "ok": false,
            "error": "name_taken"
        })));
    }

    let owner = AddressController::fetch_address(conn, owner_address, false)
        .await
        .map_err(|e| {
            error::ErrorInternalServerError(format!("Failed to fetch owner address: {}", e))
        })?
        .ok_or_else(|| error::ErrorNotFound("Owner address not found"))?;

    // TODO: Check if the user has enough balance to register the name
    // if owner.balance < state.name_cost {
    //     return Ok(HttpResponse::Ok().json(json!({
    //         "ok": false,
    //         "error": "insufficient_balance"
    //     })));
    // }

    let registration = NameRegistration {
        name,
        owner,
    };

    match NameController::register_name(conn, registration).await {
        Ok(_registered_name) => {
            // TODO: Deduct the name cost from the user's balance
            // AddressController::deduct_balance(conn, owner_address, state.name_cost).await?;

            Ok(HttpResponse::Ok().json(json!({
                "ok": true,
            })))
        }
        Err(e) => Err(error::ErrorInternalServerError(format!(
            "Failed to register name: {}",
            e
        ))),
    }
}

// https://krist.dev/docs/#api-NameGroup-GetNameCost
#[get("/cost")]
async fn get_cost_of_name(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
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
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();

    let conn = &state.conn;

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let names = NameController::get_newest_names(conn, limit, offset)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let total = NameController::name_count(conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

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
    cfg.service(get_specific_name);
    cfg.service(register_name);
    cfg.service(get_cost_of_name);
    cfg.service(get_newest_names);
}