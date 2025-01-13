use actix_web::{post, web, HttpResponse};
use nanoid::nanoid;

use crate::pg_controllers::address::AddressController;
use crate::guards::internal_key_guard;
use crate::{pg_errors::PgKromerError, AppState};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_wallets);
}

#[derive(serde::Deserialize)]
struct WalletForm {
    _username: String,
}

#[derive(serde::Serialize)]
struct WalletResponse {
    password: String,
    address: String,
}

#[post("/create", guard = "internal_key_guard")]
pub async fn create_wallets(
    state: web::Data<AppState>,
    _form: web::Json<WalletForm>,
) -> Result<HttpResponse, PgKromerError> {
    //let form = form.into_inner();
    let conn = &state.pg_db;
    // TODO: Check if username already exists
    let password = nanoid!();

    let wallet = AddressController::create_wallet(conn, password.clone()).await;
    
    let wallet = wallet.map_err(|err| PgKromerError::Database(err))?;

    let response = WalletResponse { password, address: wallet.address };

    Ok(HttpResponse::Ok().json(response))
}
