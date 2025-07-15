use axum::{
    extract::{FromRef, FromRequest, Path, State},
    response::Json,
};
use axum_extra::routing::Resource;
use serde::Serialize;
use sqlx::query;

use super::{AppState, Db};

#[derive(sqlx::FromRow, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Clone)]
pub struct Users {
    db: sqlx::SqlitePool,
}

impl FromRef<AppState> for Users {
    fn from_ref(state: &AppState) -> Self {
        let db = state.db.clone();
        Self { db }
    }
}

impl Users {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> sqlx::Result<Vec<User>> {
        query!("SELECT * FROM users").fetch_all(&self.db).await
    }

    pub async fn find_by_id(&self, id: i64) -> sqlx::Result<User> {
        query!("SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&self.db)
            .await
    }

    // ... other queries ...
}

#[axum::debug_handler]
async fn index(queries: State<Users>) -> crate::Result<Json<Vec<User>>> {
    let users = queries.all().await?;
    Ok(Json(users))
}

async fn show(queries: State<Users>, id: Path<i64>) -> crate::Result<Json<User>> {
    let user = queries.find_by_id(*id).await?;
    Ok(Json(user))
}

pub fn router() -> Resource<AppState> {
    Resource::named("users")
        .index(index) // GET /users
        .show(show) // GET /users/:id
        .into()
}
