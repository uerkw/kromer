pub mod transactions;
pub mod wallet;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(wallet::config);
    cfg.configure(transactions::config);
    // cfg.configure(transaction::config);
    // cfg.configure(name::config);
}
