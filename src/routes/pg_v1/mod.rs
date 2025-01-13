use actix_web::{web, post, HttpResponse};
use crate::pg_controllers::address::AddressController;

use crate::pg_errors::address::AddressError;
use crate::pg_errors::PgKromerError;
use crate::pg_responses::v1::LoginResponse;
use crate::AppState;

pub mod address;
pub mod lib;
pub mod name;
pub mod transaction;

#[derive(Debug, serde::Deserialize)]
struct LoginDetails {
    #[serde(rename = "privatekey")]
    private_key: String,
}


#[post("/login")]
async fn login(
    state: web::Data<AppState>,
    details: web::Json<LoginDetails>,
) -> Result<HttpResponse, PgKromerError> {
    let private_key = &details.private_key;
    let conn = &state.pg_db;

    if private_key.is_empty() {
        return Ok(HttpResponse::BadRequest().json("Private key is required"));
    }

    let owner = AddressController::verify_address(conn, private_key.to_owned() )
        .await
        .map_err(PgKromerError::Database)?
        .ok_or_else(|| PgKromerError::Address(AddressError::AuthFailed))?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        ok: true,
        authed: true,
        address: owner.address,
    }))
    
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);

    cfg.service(web::scope("/addresses").configure(address::routes));
    cfg.service(web::scope("/names").configure(name::routes));
    cfg.service(web::scope("/transactions").configure(transaction::routes));
}