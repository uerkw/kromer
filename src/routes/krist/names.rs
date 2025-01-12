use actix_web::{get, web, HttpResponse};

use crate::database::models::name::Model as Name;
use crate::errors::name::NameError;
use crate::errors::KromerError;
use crate::models::names::{NameJson, NameListResponse, NameResponse};
use crate::{routes::PaginationParams, AppState};

#[get("")]
async fn name_list(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, KromerError> {
    let params = query.into_inner();
    let db = &state.db;

    let total = Name::count(db).await?;

    let names = Name::all(db, &params).await?;
    let names: Vec<NameJson> = names.into_iter().map(|name| name.into()).collect();

    let response = NameListResponse {
        ok: true,
        count: names.len(),
        total,
        names,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{id}")]
async fn name_get(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let id = id.into_inner();
    let db = &state.db;

    let slim = Name::get_partial(db, id).await?;

    slim.map(|name| NameResponse {
        ok: true,
        name: name.into(),
    })
    .map(|response| HttpResponse::Ok().json(response))
    .ok_or_else(|| KromerError::Name(NameError::NotFound))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/names")
            .service(wallet_richest)
            .service(name_get)
            .service(name_list),
    );
}
