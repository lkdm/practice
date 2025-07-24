use axum::{
    Router,
    extract::{Extension, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::error::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CustomClaims {
    sub: String,
    email: Option<String>,
    // Custom claims here
    example: String,
}

pub async fn auth(mut req: Request, next: Next) -> crate::Result<Response> {
    debug!("started auth");
    let auth_header = match req.headers().get(http::header::AUTHORIZATION) {
        Some(header_value) => match header_value.to_str() {
            Ok(s) => s,
            Err(_) => return Err(Error::Unauthorized),
        },
        None => return Err(Error::Unauthorized),
    };

    match authorise_current_user(auth_header).await {
        Some(current_user) => {
            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        }
        None => Err(Error::Unauthorized),
    }
}
async fn authorise_current_user(auth_header: &str) -> Option<CustomClaims> {
    Some(CustomClaims {
        sub: "1".into(),
        email: Some("hello@hello.hello".into()),
        example: "A".into(),
    })
}
