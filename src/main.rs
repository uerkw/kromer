use std::env;

use actix::Actor;
use actix_web::{middleware, web, App, HttpServer};

use kromer::websockets::server::WebSocketServer;
use kromer::ws::server::WebSocketServer as NewWebSocketServer;
use surrealdb::opt::auth::Root;

use kromer::database::db::{ConnectionOptions, Database};
use kromer::{errors::KromerError, routes, AppState};

#[actix_web::main]
async fn main() -> Result<(), KromerError> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

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

    let ws_manager = WebSocketServer::new().start();

    let new_ws_manager = NewWebSocketServer::new().start();

    let state = AppState {
        db,
        ws_manager,
        new_ws_manager,
    };

    HttpServer::new(move || {
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
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(kromer::routes::config)
            .default_service(web::route().to(routes::not_found::not_found))
    })
    .bind(&server_url)?
    .run()
    .await?;

    Ok(())
}
