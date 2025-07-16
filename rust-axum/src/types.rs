use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct Identifier(#[serde(with = "ulid::serde::ulid_as_u128")] pub Ulid);

pub type DateTime = NaiveDateTime;
