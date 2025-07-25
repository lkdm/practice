use axum::{
    Extension, Json,
    extract::{FromRef, Path, State},
    response::IntoResponse,
};
use axum_extra::routing::Resource;
use chrono::{NaiveDateTime, Utc};
use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, Db,
    auth::{Claims, Permissions},
    error::Error,
    forbidden,
    types::Identifier,
    unauthorized,
};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Profile {
    id: Identifier,
    created_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    deleted_date: Option<NaiveDateTime>,
    display_name: String,
    user_id: Identifier,
}

#[derive(Deserialize)]
pub struct CreateProfile {
    display_name: String,
    user_id: Identifier,
}

#[derive(Deserialize)]
pub struct UpdateProfile {
    deleted_date: Option<NaiveDateTime>,
    display_name: String,
    user_id: Identifier,
}

#[derive(Clone)]
pub struct ProfileContext {
    db: sqlx::SqlitePool,
}

impl FromRef<AppState> for ProfileContext {
    fn from_ref(state: &AppState) -> Self {
        let db = state.db.clone();
        Self { db }
    }
}

impl ProfileContext {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> sqlx::Result<Vec<Profile>> {
        sqlx::query_as::<_, Profile>(
            r#"
                SELECT
                    id,
                    created_date,
                    modified_date,
                    deleted_date,
                    display_name,
                    user_id
                FROM
                    profile
                WHERE
                    deleted_date IS NULL OR deleted_date > CURRENT_TIMESTAMP
            "#,
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn find_by_id(&self, id: &Identifier) -> sqlx::Result<Profile> {
        sqlx::query_as::<_, Profile>(
            r#"
                SELECT
                    id,
                    created_date,
                    modified_date,
                    deleted_date,
                    display_name,
                    user_id
                FROM profile
                WHERE
                    id = ?
                    AND (deleted_date IS NULL OR deleted_date > CURRENT_TIMESTAMP)
            "#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn create(&self, payload: CreateProfile) -> sqlx::Result<Profile> {
        let now = Utc::now().naive_utc();
        sqlx::query_as::<_, Profile>(
            r#"
                INSERT INTO profile (id, created_date, modified_date, display_name, user_id)
                VALUES (?, ?, ?, ?, ?)
                RETURNING id, created_date, modified_date, deleted_date, display_name, user_id
            "#,
        )
        .bind(Identifier::new())
        .bind(now)
        .bind(now)
        .bind(payload.display_name)
        .bind(payload.user_id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn update(&self, id: Identifier, payload: UpdateProfile) -> sqlx::Result<Profile> {
        let now = Utc::now().naive_utc();
        sqlx::query_as::<_, Profile>(
            r#"
                UPDATE profile
                SET
                    deleted_date = ?,
                    modified_date = ?,
                    display_name = ?,
                    user_id = ?
                WHERE id = ?
                RETURNING id, created_date, modified_date, deleted_date, display_name, user_id
            "#,
        )
        .bind(payload.deleted_date)
        .bind(now)
        .bind(payload.display_name)
        .bind(payload.user_id)
        .bind(id)
        .fetch_one(&self.db)
        .await
    }

    pub async fn delete(&self, id: Identifier) -> sqlx::Result<()> {
        sqlx::query!(r#"DELETE FROM profile WHERE id = ?"#, id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

async fn index(
    Extension(claims): Extension<Claims>,
    queries: State<ProfileContext>,
) -> crate::Result<Json<Vec<Profile>>> {
    let p = Permissions::new(Some(&claims))?;

    match p.is_authenticated() {
        true => {}
        _ => unauthorized!(),
    }

    let profiles = queries.all().await?;
    Ok(Json(profiles))
}

async fn show(
    Extension(claims): Extension<Claims>,
    queries: State<ProfileContext>,
    id: Path<Identifier>,
) -> crate::Result<Json<Profile>> {
    let p = Permissions::new(Some(&claims))?;
    match p.is_authenticated() {
        true => {}
        _ => unauthorized!(),
    }

    let profile = queries.find_by_id(&id).await?;
    Ok(Json(profile))
}

async fn create(
    Extension(claims): Extension<Claims>,
    State(queries): State<ProfileContext>,
    Json(payload): Json<CreateProfile>,
) -> crate::Result<Json<Profile>> {
    let p = Permissions::new(Some(&claims))?;
    match p.is_authenticated() {
        true => {}
        _ => unauthorized!(),
    }
    let profile = queries.create(payload).await?;
    Ok(Json(profile))
}

async fn edit(
    method: Method,
    Extension(claims): Extension<Claims>,
    State(queries): State<ProfileContext>,
    Path(id): Path<Identifier>,
    Json(payload): Json<UpdateProfile>,
) -> crate::Result<Json<Profile>> {
    if method == axum::http::Method::PATCH {
        return Err(Error::MethodNotAllowed(method));
    }

    let profile = queries.find_by_id(&id).await?;

    let p = Permissions::new(Some(&claims))?;
    match (p.is_same_user(&profile.user_id), p.is_developer()) {
        (true, _) | (_, true) => {}
        _ => unauthorized!(),
    }

    let profile = queries.update(id, payload).await?;
    Ok(Json(profile))
}

async fn delete(
    Extension(claims): Extension<Claims>,
    State(queries): State<ProfileContext>,
    Path(id): Path<Identifier>,
) -> crate::Result<impl IntoResponse> {
    let profile = queries.find_by_id(&id).await?;

    let p = Permissions::new(Some(&claims))?;
    match (
        p.is_authenticated(),
        p.is_same_user(&profile.user_id),
        p.is_developer(),
        p.is_elevated(),
    ) {
        (false, _, _, _) => unauthorized!(),
        (_, true, _, true) | (_, _, true, true) => {}
        _ => forbidden!(),
    }

    queries.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Resource<AppState> {
    Resource::named("profiles")
        .index(index)
        .create(create)
        .show(show)
        .update(edit)
        .destroy(delete)
        .into()
}
