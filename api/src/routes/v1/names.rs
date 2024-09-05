use actix_web::{get, post, web, HttpResponse};
use kromer_economy_service::controller::{AddressController, NameController, NameRegistration};

use crate::errors::{AddressError, KromerError, NameError};
use crate::responses::v1::generic::OkResponse;
use crate::{routes::LimitAndOffset, AppState};

use crate::responses::v1::names::{Name, NameAvailabilityResponse, NameCostResponse, NameResponse, SingularNameResponse};

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

    let response: Vec<Name> = names
        .into_iter()
        .map(Into::into)
        .collect();

    let response = NameResponse {
        ok: true,
        total,
        count: response.len() as u64,
        names: response,
    };

    Ok(HttpResponse::Ok().json(response))
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

    let response = NameAvailabilityResponse {
        ok: true,
        available: is_available,
    };

    Ok(HttpResponse::Ok().json(response))
}

// https://krist.dev/docs/#api-NameGroup-GetName
#[get("/{name}")]
async fn get_specific_name(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let path_name = path.into_inner();

    let conn = &state.conn;

    let name = NameController::get_name(conn, &path_name)
        .await
        .map_err(KromerError::Database)?;

    match name {
        Some(name) => {
            let name = name.into();

            let response = SingularNameResponse {
                ok: true,
                name,
            };

            Ok(HttpResponse::Ok().json(response))
        },
        None => Err(KromerError::Name(NameError::NameNotFound(path_name))),
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
    let private_key = &body.private_key;
    let owner = AddressController::get_from_private_key(conn, private_key)
        .await
        .map_err(KromerError::Database)?
        .ok_or_else(|| KromerError::Address(AddressError::AuthFailed))?;

    let name_available = NameController::is_name_available(conn, &name)
        .await
        .map_err(KromerError::Database)?;

    if !name_available {
        return Err(KromerError::Name(NameError::NameTaken(name)));
    }

    if owner.balance < state.name_cost {
        return Err(KromerError::Name(NameError::InsufficientBalance));
    }

    let registration = NameRegistration { name, owner };

    match NameController::register_name(conn, registration).await {
        Ok(_registered_name) => {
            // TODO: Deduct the name cost from the user's balance
            // AddressController::deduct_balance(conn, owner_address, state.name_cost).await?;

            Ok(HttpResponse::Ok().json(OkResponse {
                ok: true
            }))
        }
        Err(e) => Err(KromerError::Database(e)),
    }
}

// https://krist.dev/docs/#api-NameGroup-GetNameCost
#[get("/cost")]
async fn get_cost_of_name(state: web::Data<AppState>) -> Result<HttpResponse, KromerError> {
    let name_cost = state.name_cost;

    Ok(HttpResponse::Ok().json(NameCostResponse {
        ok: true,
        cost: name_cost,
    }))
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

    let response: Vec<Name> = names
        .into_iter()
        .map(Into::into)
        .collect();

    let response = NameResponse {
        ok: true,
        total,
        count: response.len() as u64,
        names: response,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_names);
    cfg.service(get_specific_name);
    cfg.service(register_name);
    cfg.service(get_cost_of_name);
    cfg.service(get_newest_names);
    cfg.service(check_name_availability);
}
