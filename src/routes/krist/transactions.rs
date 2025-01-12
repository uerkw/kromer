use actix_web::{get, web, HttpResponse};

use crate::database::models::transaction::Model as Transaction;
use crate::errors::{transaction::TransactionError, KromerError};
use crate::models::transactions::TransactionJson;
use crate::{routes::PaginationParams, AppState};

#[get("/")]
async fn transaction_list(
    state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, KromerError> {
    let params = query.into_inner();
    let db = &state.db;

    let transactions = Transaction::all(db, &params).await?;
    let response: Vec<TransactionJson> =
        transactions.into_iter().map(|trans| trans.into()).collect();

    Ok(HttpResponse::Ok().json(response))
}

#[get("/")]
async fn transaction_latest(
    _state: web::Data<AppState>,
    _query: web::Query<PaginationParams>,
) -> Result<HttpResponse, KromerError> {
    // let params = query.into_inner();
    // let db = &state.db;

    todo!("Not yet implemented, requires sorting by date on transaction model")
}

#[get("/{id}")]
async fn transaction_get(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, KromerError> {
    let id = id.into_inner();
    let db = &state.db;

    let slim = Transaction::get_partial(db, id).await?;
    match slim {
        Some(trans) => {
            let trans: TransactionJson = trans.into();
            Ok(HttpResponse::Ok().json(trans))
        }
        None => Err(KromerError::Transaction(TransactionError::NotFound)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transactions")
            .service(transaction_list)
            .service(transaction_latest)
            .service(transaction_get),
    );
}
