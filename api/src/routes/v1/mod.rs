use actix_web::{get, post, web, Error, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier as _, PasswordHasher as _};
use kromer_economy_service::controller::AddressController;

use crate::{errors::{AuthError, KromerError}, responses::v1::LoginResponse, AppState};

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
    state: web::Data<AppState>,
    details: web::Json<LoginDetails>,
) -> Result<HttpResponse, KromerError> {
    let private_key = &details.private_key;

    if private_key.is_empty() {
        return Ok(HttpResponse::BadRequest().json("Private key is required"));
    }

    todo!()

    // let conn = &state.conn;

    // println!("{}", private_key);

    // let hasher = Argon2::default();

    // let parsed_hash = hasher.hash_password(private_key.as_bytes(), )
    //     .map_err(|_| KromerError::Auth(AuthError::AuthFailed))?;

    // println!("{}", parsed_hash.serialize().to_string());

    // let address_model = AddressController::get_from_private_key_hash(conn, &parsed_hash)
    //     .await
    //     .map_err(KromerError::Database)?;

    // println!("got address model {:?}", address_model);

    // match address_model {
    //     Some(address) => {
    //         let db_hash = PasswordHash::parse(&address.private_key, argon2::password_hash::Encoding::B64)
    //             .map_err(|_| KromerError::Auth(AuthError::AuthFailed))?;

    //         hasher
    //             .verify_password(private_key.as_bytes(), &db_hash)
    //             .map_err(|_| KromerError::Auth(AuthError::AuthFailed))?;

    //         let response = LoginResponse {
    //             ok: true,
    //             authed: true,
    //             address: address.address,
    //         };

    //         Ok(HttpResponse::Ok().json(response))
    //     }

    //     None => Err(KromerError::Auth(AuthError::AuthFailed)),
    // }
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
