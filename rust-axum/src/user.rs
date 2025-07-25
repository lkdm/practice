//! Users resource
use super::{AppState, Db};
use crate::auth::{Claims, Permissions};
use crate::unauthorized;
use crate::{error::Error, types::Identifier};
use axum::Extension;
use axum::{
    extract::{FromRef, Path, State},
    response::{IntoResponse, Json},
};
use axum_extra::routing::Resource;
use chrono::{NaiveDateTime, Utc};
use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    id: Identifier,
    created_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    deleted_date: Option<NaiveDateTime>,
    last_login_date: Option<NaiveDateTime>,
    tz: String,
    email: String,
    backup_email: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub deleted_date: Option<NaiveDateTime>,
    pub tz: String,
    email: String,
    backup_email: Option<String>,
}

#[derive(Clone)]
pub struct UserContext {
    db: sqlx::SqlitePool,
}

impl FromRef<AppState> for UserContext {
    fn from_ref(state: &AppState) -> Self {
        let db = state.db.clone();
        Self { db }
    }
}

impl UserContext {
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
                    last_login_date,
                    tz,
                    email,
                    backup_email
                FROM
                    user
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
                    deleted_date,
                    last_login_date,
                    tz,
                    email,
                    backup_email
                FROM user
                WHERE
                    id = ?
                    AND (deleted_date IS NULL OR deleted_date > CURRENT_TIMESTAMP)
            "#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn create(&self, payload: CreateUser) -> sqlx::Result<User> {
        let now = Utc::now().naive_utc();
        sqlx::query_as::<_, User>(
            r#"
                INSERT INTO user (id, created_date, modified_date, email) VALUES (?, ?, ?, ?)
                RETURNING id, created_date, modified_date, deleted_date, last_login_date, tz, email, backup_email
            "#,
        )
        .bind(Identifier::new())
        .bind(now)
        .bind(now)
        .bind(payload.email)
        .fetch_one(&self.db)
        .await
    }

    pub async fn update(&self, id: Identifier, payload: UpdateUser) -> sqlx::Result<User> {
        // Get current record
        let now = Utc::now().naive_utc();
        sqlx::query_as::<_, User>(
            r#"
                UPDATE user
                SET
                    deleted_date = ?,
                    modified_date = ?,
                    tz = ?,
                    email = ?,
                    backup_email = ?
                WHERE id = ?
                RETURNING id, created_date, modified_date, deleted_date, last_login_date, tz, email, backup_email
            "#,
        )
        .bind(payload.deleted_date)
        .bind(now)
        .bind(payload.tz)
        .bind(payload.email)
        .bind(payload.backup_email)
        .bind(id)
        .fetch_one(&self.db)
        .await
    }

    /// Hard-deletes the user and cascades to all connected records
    pub async fn delete(&self, id: Identifier) -> sqlx::Result<()> {
        sqlx::query!(r#"DELETE FROM user WHERE id = ?"#, id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

async fn index(
    Extension(claims): Extension<Claims>,
    queries: State<UserContext>,
) -> crate::Result<Json<Vec<User>>> {
    let p = Permissions::new(Some(&claims))?;
    match (p.is_developer()) {
        true => {}
        _ => unauthorized!(),
    }

    let users = queries.all().await?;
    Ok(Json(users))
}

async fn show(
    Extension(claims): Extension<Claims>,
    queries: State<UserContext>,
    id: Path<Identifier>,
) -> crate::Result<Json<User>> {
    let p = Permissions::new(Some(&claims))?;
    match (p.is_same_user(&id), p.is_developer()) {
        (true, _) | (_, true) => {}
        _ => unauthorized!(),
    }

    let user = queries.find_by_id(id.clone()).await?;
    Ok(Json(user))
}

async fn create(
    Extension(claims): Extension<Claims>,
    State(queries): State<UserContext>,
    Json(payload): Json<CreateUser>,
) -> crate::Result<Json<User>> {
    let p = Permissions::new(Some(&claims))?;
    match p.is_authenticated() {
        true => {}
        _ => unauthorized!(),
    }
    let user = queries.create(payload).await?;
    Ok(Json(user))
}

async fn edit(
    method: Method,
    Extension(claims): Extension<Claims>,
    State(queries): State<UserContext>,
    Path(id): Path<Identifier>,
    Json(payload): Json<UpdateUser>,
) -> crate::Result<Json<User>> {
    if method == axum::http::Method::PATCH {
        // Patch not implemented
        return Err(Error::MethodNotAllowed(method));
    }
    let p = Permissions::new(Some(&claims))?;
    match (p.is_same_user(&id), p.is_developer()) {
        (true, _) | (_, true) => {}
        _ => unauthorized!(),
    }

    let user = queries.update(id, payload).await?;
    Ok(Json(user))
}

async fn delete(
    Extension(claims): Extension<Claims>,
    State(queries): State<UserContext>,
    Path(id): Path<Identifier>,
) -> crate::Result<impl IntoResponse> {
    let p = Permissions::new(Some(&claims))?;
    match (p.is_same_user(&id), p.is_developer(), p.is_elevated()) {
        (true, _, true) | (_, true, true) => {}
        _ => unauthorized!(),
    }
    queries.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Resource<AppState> {
    Resource::named("users")
        .index(index)
        .create(create)
        .show(show)
        .update(edit)
        .destroy(delete)
        .into()
}
