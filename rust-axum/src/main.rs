use clap::Parser;
use std::{env, net::SocketAddr, time::Duration};

use axum::{Router, response::IntoResponse};
use http::{Method, StatusCode};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::{normalize_path::NormalizePathLayer, timeout::TimeoutLayer};

use crate::config::Config;

pub mod config;
pub mod error;
pub mod health;
pub mod types;
pub mod users;

/// Crate result type
pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;
pub type Db = sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    db: Db,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("could not start database");

    let state = AppState { db };

    let api_v1 = Router::new().merge(health::router()).merge(users::router());

    let app: Router = Router::new()
        .nest("/v1", api_v1)
        .fallback(handler_404)
        .with_state(state);

    let service: Router = app
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
                .allow_headers(tower_http::cors::Any),
        )
        // Trim trailing slash
        .layer(NormalizePathLayer::trim_trailing_slash())
        // Set a timeout for requests
        .layer(TimeoutLayer::new(Duration::from_secs(5)))
        // To mitigate DoS attacks, limit the size of request bodies.
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024 * 1024));

    let socket = SocketAddr::new(config.ip_addr, config.http_port);

    let listener = TcpListener::bind(socket)
        .await
        .expect("could not start tcp listener");

    axum::serve(listener, service)
        .await
        .expect("could not serve http service");

    Ok(())
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}
