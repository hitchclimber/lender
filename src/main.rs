mod models;
mod routes;

use actix_web::{web, App, HttpServer, Responder};
use dotenvy::dotenv;
use env_logger;
use log::{debug, error, info};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

use routes::config::config;

struct AppState {
    pool: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("✅ Connected to database");
            pool
        }
        Err(e) => {
            error!(" Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
