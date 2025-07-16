use chrono::{NaiveDate, NaiveDateTime};
use core::fmt;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::option::Option;
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct Identifier(pub Uuid);

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

struct MyDateTime(chrono::NaiveDateTime);

impl std::str::FromStr for MyDateTime {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map(MyDateTime)
    }
}
