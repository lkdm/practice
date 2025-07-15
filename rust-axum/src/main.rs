use clap::Parser;
use std::{net::SocketAddr, time::Duration};

use axum::Router;
use http::Method;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::{normalize_path::NormalizePathLayer, timeout::TimeoutLayer};

use crate::config::Config;

pub mod config;
pub mod error;
pub mod users;

/// Crate result type
pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;
pub type Db = sqlx::SqlitePool;

#[derive(Clone)]
struct AppState {
    db: Db,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();

    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("could not start database");

    let state = AppState { db };

    let app: Router = Router::new().merge(users::router()).with_state(state);

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

