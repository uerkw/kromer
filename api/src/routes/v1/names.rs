use actix_web::{get, post, web, Error, HttpResponse};

use crate::{routes::LimitAndOffset, AppState};

// https://krist.dev/docs/#api-NameGroup-GetNames
#[get("")]
async fn list_names(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, Error> {
    todo!()
}

// https://krist.dev/docs/#api-NameGroup-CheckName
#[get("/{name}/check")]
async fn check_name_availability(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    // TODO: Check if name exists, if so return that it is avaiable.
    // TODO: Return error if name doesn't exist
    todo!()
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
async fn get_cost_of_name(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // TODO: Make this cost configurable. Could use redis like Krist did or just use postgres
    //       Redis is more lightweight though.
    todo!()
}

// https://krist.dev/docs/#api-NameGroup-GetNewNames
#[get("/new")]
async fn get_newest_names(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {

    todo!()
}