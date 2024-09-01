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
            .configure(routes::routes)
    })
    .bind(&server_url)?
    .run()
    .await?;

    Ok(())
}
