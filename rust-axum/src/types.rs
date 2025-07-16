use core::fmt;
use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct Identifier(pub Ulid);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for Identifier {
    type Err = ulid::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Identifier(Ulid::from_str(s)?))
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0.into()
    }
}

struct MyDateTime(chrono::NaiveDateTime);

impl std::str::FromStr for MyDateTime {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map(MyDateTime)
    }
}
