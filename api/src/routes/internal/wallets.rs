use actix_web::{post, web, HttpResponse};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use nanoid::nanoid;
use rand::rngs::OsRng;

use crate::errors::InternalError;
use crate::util::guards::admin_key_guard;
use crate::{errors::KromerError, AppState};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_wallets);
}

#[derive(serde::Deserialize)]
struct WalletForm {
    username: String,
}

#[derive(serde::Serialize)]
struct WalletResponse {
    password: String,
}

#[post("/create", guard = "admin_key_guard")]
pub async fn create_wallets(
    _state: web::Data<AppState>,
    form: web::Json<WalletForm>,
) -> Result<HttpResponse, KromerError> {
    let form = form.into_inner();

    // TODO: Check if username already exists

    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password = nanoid!();

    let password_hash = hasher
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| KromerError::Internal(InternalError::Argon2(e.to_string())))?
        .to_string();

    // TODO: Create wallet

    let response = WalletResponse { password };

    Ok(HttpResponse::Ok().json(response))
}
