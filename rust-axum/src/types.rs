use core::fmt;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::types::Uuid;
use std::option::Option;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, sqlx::Type, Clone)]
#[sqlx(transparent)]
/// A newtype abstraction over UUID
pub struct Identifier(pub Uuid);

impl Identifier {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0.into()
    }
}

impl From<Uuid> for Identifier {
    fn from(uuid: Uuid) -> Self {
        Identifier(uuid)
    }
}
