use clap::Parser;
use std::{env, net::SocketAddr, time::Duration};
use tracing::{Level, debug};

use axum::{Router, middleware, response::IntoResponse};
use http::{Method, StatusCode};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};

use crate::config::Config;

pub mod auth;
pub mod config;
pub mod error;
pub mod health;
pub mod profile;
pub mod types;
pub mod user;

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

    tracing_subscriber::fmt()
        .with_target(false) // Optional: suppress target field
        .with_max_level(Level::DEBUG)
        .compact() // Optional: compact output
        .init();

    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&config.database_url)
        .await
        .expect("could not start database");

    let state = AppState { db };

    // Routes that are protected by authentication
    let protected_routes = Router::new()
        .merge(user::router())
        .merge(profile::router())
        .route_layer(middleware::from_fn(auth::auth));

    // Routes that are not protected by authentication
    let unprotected_routes = Router::new().merge(health::router());

    // API version 1
    let api_v1 = Router::new()
        .merge(unprotected_routes)
        .merge(protected_routes);

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
        // TODO: Add auth layer
        // Trim trailing slash
        .layer(NormalizePathLayer::trim_trailing_slash())
        // Set a timeout for requests
        .layer(TimeoutLayer::new(Duration::from_secs(5)))
        // To mitigate DoS attacks, limit the size of request bodies.
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024 * 1024))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let socket = config
        .api_url
        .socket_addrs(|| None)
        .expect("failed to resolve socket address from api_url")
        .into_iter()
        .next()
        .expect("no socket address found in api_url");

    let listener = TcpListener::bind(socket)
        .await
        .expect("could not start tcp listener");

    tracing::info!("Server listening on http://{}", socket);

    axum::serve(listener, service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
            tracing::info!("Shutdown signal received, server is shutting down");
        })
        .await
        .expect("could not serve http service");

    Ok(())
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}
