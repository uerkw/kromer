mod routes;

use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::env;

use kromer_economy_migration::{Migrator, MigratorTrait};
use kromer_economy_service::sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub name_cost: u64,
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hewwo!!")
}

async fn not_found(request: HttpRequest) -> HttpResponse {
    let body = format!("Resource {} not found.", request.uri().path());

    HttpResponse::NotFound().body(body)
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

    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState {
        conn,
        name_cost: 500,
    };

    HttpServer::new(move || {
        App::new()
            // .service(Fs::new("/static", "./api/static"))
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .default_service(web::route().to(not_found))
            .service(hello)
            .service(
                // TODO: Use `configure` instead, https://actix.rs/docs/application/#configure
                web::scope("/api/v1")
                    .service(
                        web::scope("/addresses")
                            .service(routes::v1::addresses::list_addresses)
                            .service(routes::v1::addresses::get_richest_addresses) // This has to be here otherwise /addresses/rich will conflict with /addresses/:address
                            .service(routes::v1::addresses::get_specific_address)
                            .service(routes::v1::addresses::get_address_names)
                            .service(routes::v1::addresses::get_address_transactions),
                    )
                    .service(
                        web::scope("/names")
                            .service(routes::v1::names::list_names)
                            .service(routes::v1::names::check_name_availability)
                            .service(routes::v1::names::get_specific_name)
                            .service(routes::v1::names::register_name)
                            .service(routes::v1::names::get_newest_names)
                            .service(routes::v1::names::get_cost_of_name),
                    )
                    .service(routes::v1::login) // This does not work?
                    .service(routes::v1::motd)
                    .service(routes::v1::walletversion)
                    .service(routes::v1::whats_new)
                    .service(routes::v1::kromer_supply)
                    .service(routes::v1::get_v2_wallet),
            )
    })
    .bind(&server_url)?
    .run()
    .await?;

    Ok(())
}
