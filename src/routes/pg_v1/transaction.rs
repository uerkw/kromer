use actix_web::{get, post, web, HttpResponse, Result};

use crate::pg_errors::transaction::TransactionError;
use crate::pg_responses::v1::transaction::{
    SingularTransactionResponse, Transaction, TransactionResponse,
};
use crate::{pg_errors::PgKromerError, routes::pg_v1::lib::LimitAndOffset, AppState};
use crate::pg_controllers::transaction::TransactionController;

// https://krist.dev/docs/#api-TransactionGroup-GetTransactions
#[get("")]
async fn list_transactions(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, PgKromerError> {
    let query = query.into_inner();
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let conn = &state.pg_db;

    let transactions = TransactionController::all(conn, limit, offset)
        .await
        .map_err(PgKromerError::Database)?;

    let total = TransactionController::total(conn)
        .await
        .map_err(PgKromerError::Database)?;

    let response: Vec<Transaction> = transactions.into_iter().map(Into::into).collect();

    let response = TransactionResponse {
        ok: true,
        total,
        count: response.len() as u64,
        transactions: response,
    };

    Ok(HttpResponse::Ok().json(response))
}

// https://krist.dev/docs/#api-TransactionGroup-GetLatestTransactions
#[get("/latest")]
async fn list_latest_transactions(
    state: web::Data<AppState>,
    query: web::Query<LimitAndOffset>,
) -> Result<HttpResponse, PgKromerError> {
    let query = query.into_inner();
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let conn = &state.pg_db;

    let transactions = TransactionController::latest(conn, limit, offset)
        .await
        .map_err(PgKromerError::Database)?;

    let total = TransactionController::total(conn)
        .await
        .map_err(PgKromerError::Database)?;

    let response: Vec<Transaction> = transactions.into_iter().map(Into::into).collect();

    let response = TransactionResponse {
        ok: true,
        total,
        count: response.len() as u64,
        transactions: response,
    };

    Ok(HttpResponse::Ok().json(response))
}

// https://krist.dev/docs/#api-TransactionGroup-GetSpecificTransaction
#[get("/{transaction_id}")]
async fn get_specific_transaction(
    state: web::Data<AppState>,
    path: web::Path<i32>, // Guh, cant use u64.
) -> Result<HttpResponse, PgKromerError> {
    let transaction_id = path.into_inner();

    let conn = &state.pg_db;

    let transaction = TransactionController::get_by_id(conn, transaction_id)
        .await
        .map_err(PgKromerError::Database)?;

    match transaction {
        Some(tx) => {
            let tx: Transaction = tx.into(); // God i love rust <3

            let response = SingularTransactionResponse {
                ok: true,
                transaction: tx,
            };

            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(PgKromerError::Transaction(TransactionError::NotFound)),
    }
}

#[post("/")]
async fn make_transaction() -> Result<HttpResponse, PgKromerError> {
    Ok(HttpResponse::Ok().body("Hewwo!!"))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_transactions);
    cfg.service(list_latest_transactions);
    cfg.service(get_specific_transaction);
    cfg.service(make_transaction);
}