use std::str::FromStr;

use axum::{
    Router,
    body::Body,
    extract::{Extension, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;

use crate::{
    error::{DeveloperError, Error},
    types::Identifier,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Developer,
    Admin,
    User,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
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
async fn authorise_current_user(auth_header: &str) -> Option<Claims> {
    Some(Claims {
        sub: "1".into(),
        email: Some("hello@hello.hello".into()),
        example: "A".into(),
    })
}

async fn protected_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    // Now you can access claims fields like claims.sub, claims.email, etc.
    format!("Hello user: {}, email: {:?}", claims.sub, claims.email)
}

/// Permissions builder
///
/// Allows the developer to easily build a set of required permissions and check them in-line.
#[derive(Clone)]
pub struct Permissions {
    claimed_id: Option<Identifier>,
}

impl Permissions {
    pub fn new(claims: Option<&Claims>) -> crate::Result<Self> {
        let mut validation_errors: Vec<(String, String)> = Vec::new();

        let claimed_id = match claims {
            Some(claims) => match claims.sub.parse::<Identifier>() {
                Ok(id) => Some(id),
                Err(_) => {
                    validation_errors.push(("sub".into(), "invalid user sub".into()));
                    None
                }
            },
            None => None, // no claims → anonymous
        };

        if validation_errors.len() > 0 {
            return Err(Error::unprocessable_entity(validation_errors));
        }

        Ok(Self { claimed_id })
    }

    pub fn is_same_user(&self, required_id: &Identifier) -> bool {
        self.claimed_id
            .as_ref()
            .map_or(false, |id| id == required_id)
    }

    pub fn is_authenticated(&self) -> bool {
        self.claimed_id.is_some()
    }

    pub fn is_unauthenticated(&self) -> bool {
        self.claimed_id.is_none()
    }

    pub fn is_developer(&self) -> bool {
        // TODO: Insert real developer UUID here
        let developer_id = Identifier::new();
        self.is_same_user(&developer_id)
    }
}

#[macro_export]
macro_rules! unauthorized {
    ($msg:expr) => {{
        tracing::warn!("Unauthorized attempt: {}", $msg);
        return Err(crate::error::Error::Unauthorized);
    }};

    () => {
        return Err(crate::error::Error::Unauthorized)
    };
}
