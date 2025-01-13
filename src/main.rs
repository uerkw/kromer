use std::env;
use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};

use kromer::websockets::token_cache::TokenCache;
use kromer::websockets::ws_manager::WsDataManager;
use kromer::websockets::ws_server::WsServer;
use surrealdb::opt::auth::Root;

use kromer::database::db::{ConnectionOptions, Database};
use kromer::{errors::KromerError, routes, AppState};
use tokio::sync::Mutex;
use tokio::{spawn, try_join};

use sea_orm::Database as SeaDatabase;

#[actix_web::main]
async fn main() -> Result<(), KromerError> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let pg_database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL is not set in .env file");

    let pg_db = SeaDatabase::connect(&pg_database_url)
        .await
        .expect("Failed to connect to the Postgres database");

    // TODO: Factor the database stuff out to a function.
    let surreal_endpoint = env::var("SURREAL_URL").expect("SURREAL_URL is not set in .env file");
    let surreal_user = env::var("SURREAL_USER").expect("SURREAL_USER is not set in .env file");
    let surreal_password =
        env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD is not set in .env file");
    let surreal_namespace =
        env::var("SURREAL_NAMESPACE").expect("SURREAL_NAMESPACE is not set in .env file");
    let surreal_database =
        env::var("SURREAL_DATABASE").expect("SURREAL_DATABASE is not set in .env file");

    let connect_options = ConnectionOptions {
        namespace: &surreal_namespace,
        database: &surreal_database,
        credentials: Root {
            username: &surreal_user,
            password: &surreal_password,
        },
    };

    let db = Database::connect(&surreal_endpoint, &connect_options).await?;

    let db_arc = Arc::new(db);


    let (ws_server, ws_server_handle) = WsServer::new();
    let ws_server = spawn(ws_server.run());
    let token_cache = Arc::new(Mutex::new(TokenCache::new()));
    let ws_manager = Arc::new(Mutex::new(WsDataManager::default()));

    let state = web::Data::new(AppState {
        pg_db,
        db: db_arc,
        name_cost: 500.0,
        ws_server_handle,
        token_cache,
        ws_manager,
    });

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(
                web::FormConfig::default()
                    .error_handler(|err, _req| KromerError::Validation(err.to_string()).into()),
            )
            .app_data(
                web::QueryConfig::default()
                    .error_handler(|err, _req| KromerError::Validation(err.to_string()).into()),
            )
            .app_data(
                web::PathConfig::default()
                    .error_handler(|err, _req| KromerError::Validation(err.to_string()).into()),
            )
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|err, _req| KromerError::Validation(err.to_string()).into()),
            )
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(kromer::routes::config)
            .default_service(web::route().to(routes::not_found::not_found))
    })
    .bind(&server_url)?
    .run();

    // Join the tasks together using tokio.
    try_join!(http_server, async move { ws_server.await.unwrap() })?;

    Ok(())
}
