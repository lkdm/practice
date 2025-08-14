use thiserror::Error;

use crate::share::PluginRegistrationError;

pub type Result<T> = std::result::Result<T, Error>;
pub trait ShareError: std::error::Error + Send + Sync + 'static {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("filesystem error: {0}")]
    IO(#[from] std::io::Error),

    #[error("plugin registration error: {0}")]
    Registration(#[from] PluginRegistrationError),

    /// ShareError
    ///
    /// Any Error that implements `ShareError` can be wrapped in this.
    ///
    /// ## Usage
    /// ```rs
    /// impl ShareError for MyError {}
    /// ```
    #[error("share plugin failed: {0}")]
    Share(Box<dyn ShareError>),
}

impl<E: ShareError> From<E> for Error {
    fn from(err: E) -> Self {
        Error::Share(Box::new(err))
    }
}
