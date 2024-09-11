pub mod errors;
pub mod responses;
mod routes;
mod util;

use actix_governor::{Governor, GovernorConfigBuilder, KeyExtractor, SimpleKeyExtractionError};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, ResponseError, Result};
use std::{env, net::IpAddr};

use errors::KromerError;
use kromer_economy_migration::{Migrator, MigratorTrait};
use kromer_economy_service::sea_orm::{Database, DatabaseConnection};
use util::cors::default_cors_config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub name_cost: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct KromerPeerIpExtractor;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hewwo!!")
}

// Unfortunate how HttpResponse is required, lol
async fn not_found() -> Result<HttpResponse, KromerError> {
    Err(KromerError::Routes(errors::RoutesError::NotFound))
}

#[actix_web::main]
pub async fn start() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(12)
        .key_extractor(KromerPeerIpExtractor)
        .finish()
        .expect("Failed to create governor config");

    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState {
        conn,
        name_cost: 500.0,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(default_cors_config())
            .app_data(web::Data::new(state.clone()))
            .wrap(Governor::new(&governor_conf))
            .wrap(middleware::Logger::default()) // enable logger
            .default_service(web::route().to(not_found))
            .service(hello)
            .configure(routes::routes)
    })
    .bind(&server_url)?
    .run()
    .await?;

    Ok(())
}

impl KeyExtractor for KromerPeerIpExtractor {
    type Key = IpAddr;
    type KeyExtractionError = SimpleKeyExtractionError<KromerError>;

    fn extract(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> std::result::Result<Self::Key, Self::KeyExtractionError> {
        req.peer_addr().map(|socket| socket.ip()).ok_or_else(|| {
            SimpleKeyExtractionError::new(KromerError::Routes(errors::RoutesError::RateLimitHit))
        })
    }

    fn exceed_rate_limit_response(
        &self,
        _negative: &actix_governor::governor::NotUntil<
            actix_governor::governor::clock::QuantaInstant,
        >,
        mut _response: actix_web::HttpResponseBuilder,
    ) -> HttpResponse {
        errors::RoutesError::RateLimitHit.error_response()
    }
}