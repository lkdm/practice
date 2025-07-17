use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::{convert::TryFrom, fmt};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Type, Clone)]
#[sqlx(transparent)]
/// Made a choice to store the Identifier as a String, but parse it as a UUID, because SQLite is
/// too type-permissive.
pub struct Identifier(String);

impl Identifier {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0
    }
}

impl TryFrom<String> for Identifier {
    type Error = uuid::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&s)?; // Validation
        Ok(Identifier(s))
    }
}
