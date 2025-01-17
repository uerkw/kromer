use actix_web::{get, web, HttpResponse};

use crate::database::models::transaction::Model as Transaction;
use crate::errors::krist::{transaction::TransactionError, KristError};
use crate::models::transactions::{TransactionJson, TransactionListResponse, TransactionResponse};
use crate::{routes::PaginationParams, AppState};

#[get("")]
async fn transaction_list(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    let params = query.into_inner();
    let db = &state.db;

    let total = Transaction::count(db).await?;

    let transactions = Transaction::all(db, &params).await?;
    let transactions: Vec<TransactionJson> =
        transactions.into_iter().map(|trans| trans.into()).collect();

    let response = TransactionListResponse {
        ok: true,
        count: transactions.len(),
        total,
        transactions,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/latest")]
async fn transaction_latest(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, KristError> {
    let params = query.into_inner();
    let db = &state.db;

    let total = Transaction::count(db).await?;
    let transactions = Transaction::sorted_by_date(db, &params).await?;

    let transactions: Vec<TransactionJson> =
        transactions.into_iter().map(|trans| trans.into()).collect();

    let response = TransactionListResponse {
        ok: true,
        count: transactions.len(),
        total,
        transactions,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{id}")]
async fn transaction_get(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, KristError> {
    let id = id.into_inner();
    let db = &state.db;

    let slim = Transaction::get_partial(db, id).await?;

    slim.map(|trans| TransactionResponse {
        ok: true,
        transaction: trans.into(),
    })
    .map(|response| HttpResponse::Ok().json(response))
    .ok_or_else(|| KristError::Transaction(TransactionError::NotFound))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transactions")
            .service(transaction_latest)
            .service(transaction_get)
            .service(transaction_list),
    );
}
