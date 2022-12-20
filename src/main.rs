mod app;
mod controllers;
mod database;
mod infra;
mod interactors;
mod models;
mod repositories;
mod usecases;

use dotenvy;
use std::{env, net::SocketAddr};

use app::PgContext;
use database::ConnectionPoolOptions;

#[tokio::main]
async fn main() {
    let app_env = match env::var("APP_ENV") {
        Ok(app_env) => app_env,
        Err(_) => "local".to_string(),
    };

    if app_env == "production" {
        dotenvy::dotenv().ok();
    } else {
        dotenvy::from_filename(format!(".env.{}", app_env)).ok();
    }

    let max_connections = match env::var("DB_CONNECTIONS") {
        Ok(v) => v.parse::<u32>().unwrap(),
        Err(_) => 5,
    };

    let connect_options = ConnectionPoolOptions {
        max_connections: max_connections,
        user: env::var("DB_USER").unwrap_or("root".to_string()),
        password: env::var("DB_PASSWORD").unwrap_or("password".to_string()),
        database: env::var("DB_NAME").unwrap_or("postgres".to_string()),
        host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
        port: env::var("DB_PORT").unwrap_or("5432".to_string()),
    };

    let conn = database::connect(&connect_options)
        .await
        .expect("Connection failed");

    let context = PgContext::new(conn);

    let app = app::create_router(context);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
