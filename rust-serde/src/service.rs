use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Service
///
/// A service represents a running component of the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// Process ID used to keep track of the Service
    pid: u32,
    /// Human-readable name of the process
    name: String,
    /// The datetime the service was started, in UTC
    #[serde(with = "chrono::serde::ts_seconds")]
    start_time: DateTime<Utc>,
}

impl Service {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            name: name.into(),
            start_time: Utc::now(),
        }
    }
}
