use actix_web::web;

use crate::guards;

use crate::websockets::payload_ws;

pub mod index;
pub mod internal;
pub mod new_ws;
pub mod not_found;
pub mod v1;
pub mod ws;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PaginationParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: Some(50),
            offset: Some(0),
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").configure(v1::config));
    cfg.service(
        web::scope("/api/_internal")
            .guard(guards::internal_key_guard)
            .configure(internal::config),
    );
    cfg.service(
        web::scope("/ws")
            .service(ws::request_token)
            .service(web::resource("/gateway/{token}").to(payload_ws)),
    );
    cfg.service(
        web::scope("/new_ws")
            //.service(new_ws::request_token)
            .service(new_ws::payload_ws),
    );
    cfg.service(web::scope("").service(index::index_get));
}
