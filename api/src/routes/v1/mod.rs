use actix_web::{get, post, web, Error, HttpResponse};

use crate::AppState;

pub mod addresses;
pub mod names;
pub mod transactions;
pub mod ws;

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
async fn motd(_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[get("/walletversion")]
async fn walletversion(_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[get("/whatsnew")]
async fn whats_new(_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[get("/supply")]
async fn kromer_supply(_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

#[post("/v2")]
async fn get_v2_wallet(
    _state: web::Data<AppState>,
    _details: web::Json<LoginDetails>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(motd);
    cfg.service(walletversion);
    cfg.service(whats_new);
    cfg.service(kromer_supply);
    cfg.service(get_v2_wallet);
    cfg.service(web::resource("/ws").route(web::get().to(ws::websocket)));

    cfg.service(web::scope("/addresses").configure(addresses::routes));
    cfg.service(web::scope("/names").configure(names::routes));
    cfg.service(web::scope("/transactions").configure(transactions::routes));
}
