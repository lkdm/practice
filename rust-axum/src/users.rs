//! Users resource
use super::{AppState, Db};
use crate::types::Identifier;
use axum::{
    extract::{FromRef, Path, State},
    response::{IntoResponse, Json},
};
use axum_extra::routing::Resource;
use chrono::NaiveDateTime;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    id: Identifier,
    created_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    deleted_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub deleted_date: Option<NaiveDateTime>,
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
                    deleted_date
                FROM
                    users
                WHERE
                    deleted_date IS NULL OR deleted_date > CURRENT_TIMESTAMP
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
                    deleted_date
                FROM users
                WHERE
                    id = ?
                    AND (deleted_date IS NULL OR deleted_date > CURRENT_TIMESTAMP)
            "#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn create(&self) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (id) VALUES (?)
                RETURNING id, created_date, modified_date, deleted_date
            "#,
        )
        .bind(Identifier::new())
        .fetch_one(&self.db)
        .await
    }

    pub async fn update(&self, id: Identifier, payload: UpdateUser) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>(
            r#"
                UPDATE users
                SET deleted_date = ?
                WHERE id = ?
                RETURNING id, created_date, modified_date, deleted_date
            "#,
        )
        .bind(payload.deleted_date)
        .bind(id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn delete(&self, id: Identifier) -> sqlx::Result<()> {
        sqlx::query!(r#"DELETE FROM users WHERE id = ?"#, id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

async fn index(queries: State<Users>) -> crate::Result<Json<Vec<User>>> {
    let users = queries.all().await?;
    Ok(Json(users))
}

async fn show(queries: State<Users>, id: Path<Identifier>) -> crate::Result<Json<User>> {
    let user = queries.find_by_id(id.clone()).await?;
    Ok(Json(user))
}

async fn create(queries: State<Users>) -> crate::Result<Json<User>> {
    let user = queries.create().await?;
    Ok(Json(user))
}

async fn edit(
    State(queries): State<Users>,
    Path(id): Path<Identifier>,
    Json(payload): Json<UpdateUser>,
) -> crate::Result<Json<User>> {
    let user = queries.update(id, payload).await?;
    Ok(Json(user))
}

async fn delete(
    State(queries): State<Users>,
    Path(id): Path<Identifier>,
) -> crate::Result<impl IntoResponse> {
    queries.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Resource<AppState> {
    Resource::named("users")
        .index(index) // GET /users
        .create(create) // POST /users
        .show(show) // GET /users/:id
        .update(edit) // PATCH /users/:id
        .destroy(delete) // DELETE /users/:id
        .into()
}
