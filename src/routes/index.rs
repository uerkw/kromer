use actix_web::{get, HttpResponse};

use crate::errors::KromerError;

#[get("/")]
pub async fn index_get() -> Result<HttpResponse, KromerError> {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}
