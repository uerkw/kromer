pub mod wallet;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(wallet::config);
}
