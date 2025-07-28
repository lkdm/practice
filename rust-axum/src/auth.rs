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
use chrono::{Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tower::ServiceBuilder;
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
/// Claims
///
/// Claims should reflect mainly identity, and ownership, but not permissions or state.
///
/// TODO: https://lib.rs/install/jsonwebtoken
pub struct Claims {
    sub: String,
    email: Option<String>,
    exp: usize, // Owned profiles
                // profile_ids: Vec<String>,
}

/// check_authentication
///
/// Asks: Is the subject who they claim to be?
///
/// Steps:
/// 1. Decodes and verififies JWT token and claims (such as expiration).
/// 2. Rejects requests with invalid, expired, or tampered tokens.
/// 3. If valid, extracts the claims and attaches it to the request context.
pub async fn check_authentication(mut req: Request, next: Next) -> crate::Result<Response> {
    debug!("started auth");
    let auth_header = match req.headers().get(http::header::AUTHORIZATION) {
        Some(header_value) => match header_value.to_str() {
            Ok(s) => s,
            Err(_) => return Err(Error::Unauthorized),
        },
        None => return Err(Error::Unauthorized),
    };

    let claims = Claims {
        sub: "1".to_string(),
        email: Some("hello@hello.hello".to_string()),
    };

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

/// check_authorisation
///
/// Asks whether the user is allowed to access the resource?
/// 1. Checks permission rules
pub async fn check_authorisation(
    Extension(claims): Extension<Claims>,
    mut req: Request,
    next: Next,
) -> crate::Result<Response> {
    // Err(Error::Forbidden) // ONLY if you want to show that the resource exists
    // Err(Error::NotFound) // If you want to hide the fact that the resource exists
    // /// Are they they the correct user but require elevation?
    // /// Must send back WWW-Authenticate header
    // Err(Error::Unauthorized)
    // todo!("is the user who they claim to be?")
    debug!("Checking permissions for claims: {:?}", claims);
    Ok(next.run(req).await)
}

/// Permissions builder
///
/// Allows the developer to easily build a set of required permissions and check them in-line.
///
/// Three levels of permissions checks:
/// 1. Authentication: Is the subject who they claim to be?
/// 2. Authorisation to enter handler: Is the handler callable by the subject?
/// 3. Authorisation to access the resource: Is the resource accessible to the subject?
///
/// TODO: The first two should be middleware and overlap
#[derive(Clone)]
pub struct Permissions {
    claimed_id: Option<Identifier>,
    is_elevated: bool,
}

impl Permissions {
    pub fn new(claims: Option<&Claims>) -> crate::Result<Self> {
        if claims.is_none() {
            return Ok(Permissions {
                claimed_id: None,
                is_elevated: false,
            });
        }
        let claims = claims.unwrap();
        let mut validation_errors: Vec<(String, String)> = Vec::new();

        let claimed_id = match claims.sub.parse::<Identifier>() {
            Ok(id) => Some(id),
            Err(_) => {
                validation_errors.push(("sub".into(), "invalid user sub".into()));
                None
            }
        };

        // let is_elevated = claims.mfa_recent_at.map_or(false, |mfa_time| {
        //     let now = Utc::now().naive_utc();
        //     now.signed_duration_since(mfa_time) <= Duration::minutes(5)
        // });
        let is_elevated = true;

        if validation_errors.len() > 0 {
            return Err(Error::unprocessable_entity(validation_errors));
        }

        Ok(Self {
            claimed_id,
            is_elevated,
        })
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

    pub fn is_elevated(&self) -> bool {
        self.is_elevated
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

#[macro_export]
macro_rules! forbidden {
    ($msg:expr) => {{
        tracing::warn!("Forbidden attempt: {}", $msg);
        return Err(crate::error::Error::Forbidden);
    }};

    () => {
        return Err(crate::error::Error::Forbidden)
    };
}
