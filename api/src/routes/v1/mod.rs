use actix_web::{get, post, web, Error, HttpResponse};

use crate::AppState;

pub mod addresses;

#[derive(Debug, serde::Deserialize)]
struct LoginDetails {
    #[serde(rename = "privatekey")]
    private_key: String,
}

// https://krist.dev/docs/#api-MiscellaneousGroup-Login
#[post("/login")]
async fn login(
    _state: web::Data<AppState>,
    _details: web::Json<LoginDetails>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

// https://krist.dev/docs/#api-MiscellaneousGroup-GetMOTD
#[get("/motd")]
async fn motd(
    _state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[get("/walletversion")]
async fn walletversion(
    _state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[get("/whatsnew")]
async fn whats_new(
    _state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[get("/supply")]
async fn kromer_supply(
    _state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[post("/v2")]
async fn get_v2_wallet(
    _state: web::Data<AppState>,
    _details: web::Json<LoginDetails>
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}