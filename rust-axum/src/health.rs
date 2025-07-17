//! Health check API module
use crate::{AppState, Db};
use axum::extract::State;
use axum::{Json, extract::FromRef};
use axum_extra::routing::Resource;
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    api: bool,
    db: bool,
}

#[derive(Clone)]
pub struct HealthChecks {
    db: sqlx::SqlitePool,
}

impl FromRef<AppState> for HealthChecks {
    fn from_ref(state: &AppState) -> Self {
        let db = state.db.clone();
        Self { db }
    }
}

impl HealthChecks {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    /// Check DB connectivity asynchronously
    pub async fn check(&self) -> bool {
        sqlx::query("SELECT 1").execute(&self.db).await.is_ok()
    }
}

async fn health_handler(State(health_checks): State<HealthChecks>) -> Json<Health> {
    let db_ok = health_checks.check().await;
    Json(Health {
        api: true,
        db: db_ok,
    })
}

pub fn router() -> Resource<AppState> {
    Resource::named("health")
        .index(health_handler) // GET /users
        .into()
}
