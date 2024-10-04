use actix_web::{post, web, HttpResponse};
use serde_json::json;

use crate::database::models::player::Model as Player;
use crate::database::models::wallet::Model as Wallet;
use crate::errors::transaction::TransactionError;
use crate::errors::wallet::WalletError;
use crate::{errors::KromerError, AppState};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct MinecraftUser {
    pub name: String,
    pub mc_uuid: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct GiveMoneyReq {
    pub address: String,
    pub amount: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Guh {
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct WalletCreateResp {
    pub address: String,
    pub password: String,
    pub wallet: Wallet,
}

#[post("/create")]
async fn wallet_create(
    state: web::Data<AppState>,
    user: web::Json<MinecraftUser>,
) -> Result<HttpResponse, KromerError> {
    // fn::create_wallet_with_user(username)
    let db = &state.db;
    let user = user.into_inner();

    let player: Option<Player> = db
        .create(("player", user.mc_uuid))
        .content(Guh { name: user.name })
        .await?;
    let player = player.ok_or_else(|| KromerError::Internal("Unable to get created player"))?;

    let mut resp = db.query("RETURN fn::create_wallet(100);").await?;
    let wallet: Option<WalletCreateResp> = resp.take(0)?;
    let wallet = wallet.ok_or_else(|| KromerError::Internal("Unable to get created wallet"))?;

    let q = "RELATE $player->owns->$wallet";
    let resp = db
        .query(q)
        .bind(("player", player.id.unwrap()))
        .bind(("wallet", wallet.wallet.id.unwrap()))
        .await?;
    tracing::debug!("Got response: {resp:?}");

    // Yeah i dont like this either
    let resp = json!({
        "password": wallet.password,
        "address": wallet.address
    });

    Ok(HttpResponse::Ok().json(resp))
}

#[post("/give-money")]
async fn wallet_give_money(
    state: web::Data<AppState>,
    data: web::Json<GiveMoneyReq>,
) -> Result<HttpResponse, KromerError> {
    let db = &state.db;
    let data = data.into_inner();

    if data.amount < 0.0 {
        return Err(KromerError::Transaction(TransactionError::InvalidAmount));
    }

    let wallet = Wallet::get_by_address(db, data.address)
        .await?
        .ok_or(KromerError::Wallet(WalletError::NotFound))?;
    let q = "UPDATE $wallet SET balance += $amount";
    let _ = db
        .query(q)
        .bind(("wallet", wallet.id.unwrap()))
        .bind(("amount", data.amount))
        .await?;

    let resp = json!({
        "ok": true
    });

    Ok(HttpResponse::Ok().json(resp))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/wallet")
            .service(wallet_create)
            .service(wallet_give_money),
    );
}
