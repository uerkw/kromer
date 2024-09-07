use actix_web::web;

pub mod internal;
pub mod v1;

#[derive(Debug, serde::Deserialize)]
struct LimitAndOffset {
    limit: Option<u64>,
    offset: Option<u64>,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/_internal").configure(internal::config));
    cfg.service(web::scope("/api/v1").configure(v1::routes));
}
