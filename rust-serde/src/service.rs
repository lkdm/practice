use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::fs::{self, BinaryFileError};

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

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("binary file error: {0}")]
    BinaryFile(#[from] BinaryFileError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Services(Vec<Service>);

impl Services {
    pub fn new(services: &[Service]) -> Self {
        Self(services.to_vec())
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new(&[
            Service::new(100, "one hundred"),
            Service::new(200, "two hundred"),
        ])
    }
}

/// ServicePersistence
///
/// Trait for writing and reading services
pub trait ServicePersistence: Clone + 'static {
    /// Persist the services
    fn write_services(&self, req: &Services) -> std::result::Result<(), ServiceError>;
    fn read_services(&self) -> std::result::Result<Services, ServiceError>;
}
