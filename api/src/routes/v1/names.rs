use actix_web::{error, get, post, web, Error, HttpResponse};
use kromer_economy_service::controller::NameController;
use serde_json::json;

use crate::{routes::LimitAndOffset, AppState};

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

    let total = NameController::count(conn)
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

    // TODO: Check if name exists, if so return it.
    // TODO: Return error if name doesn't exist
    todo!()
}

// https://krist.dev/docs/#api-NameGroup-RegisterName
#[post("/{name}")]
async fn register_name(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    // TODO: Check if name exists, if so return an error.
    // TODO: Create the name if it doesn't exist.
    todo!()
}

// https://krist.dev/docs/#api-NameGroup-GetNameCost
#[get("/cost")]
async fn get_cost_of_name(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // TODO: Make this cost configurable. Could use redis like Krist did or just use postgres
    //       Redis is more lightweight though.
    todo!()
}

// https://krist.dev/docs/#api-NameGroup-GetNewNames
#[get("/new")]
async fn get_newest_names(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    todo!()
}
