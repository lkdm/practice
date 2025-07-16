use crate::types::Identifier;
use axum::{
    extract::{FromRef, Path, State},
    response::Json,
};
use axum_extra::routing::Resource;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use super::{AppState, Db};

#[derive(Debug, sqlx::FromRow, Serialize)]
struct User {
    id: Identifier,
    created_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    deleted_date: Option<NaiveDateTime>,
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
        sqlx::query_as::<_, User>(
            r#"
                SELECT
                    id,
                    created_date,
                    modified_date,
                    deleted_date,
                FROM
                    users
            "#,
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn find_by_id(&self, id: Identifier) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT 
                    id,
                    created_date,
                    modified_date,
                    deleted_date,
                FROM users WHERE id = ?
            "#,
        )
        .bind(id)
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

async fn show(queries: State<Users>, id: Path<Identifier>) -> crate::Result<Json<User>> {
    let user = queries.find_by_id(id.clone()).await?;
    Ok(Json(user))
}

pub fn router() -> Resource<AppState> {
    Resource::named("users")
        .index(index) // GET /users
        .show(show) // GET /users/:id
        .into()
}
