use ropey::Rope;
use thiserror::Error;
use uuid::Uuid;

type Timestamp = chrono::NaiveDateTime;

struct DraftId(Uuid);

struct Text(Rope);

enum DraftState {
    /// Scratchpad drafts are the default, they are archived after being shared
    Scratchpad,
    /// Archived drafts are deleted after `n` days
    Archived,
    /// Deleted drafts are permanently deleted
    Deleted,
    /// Reference drafts stick around after being shared
    Reference,
}

pub struct Draft {
    id: DraftId,
    text: Text,
    created_at: Timestamp,
    state: DraftState,
}

struct ShareMetadata {
    shared_at: Option<Timestamp>,
    // method: ShareMethod,
}

pub trait PluginError: std::error::Error + Send + Sync + 'static {}

#[derive(Debug, Error)]
pub enum ShareError {
    /// Internal server error
    ///
    /// Any Error that implements `InternalError` will automatically use this.
    ///
    /// An alternative to this is [`anyhow`], but this allows you to statically define your error
    /// variants.
    ///
    /// ## Usage
    /// ```rs
    /// impl InternalError for MyError {}
    /// ```
    #[error("an internal server error occurred")]
    Plugin(Box<dyn InternalError>),
}

trait ShareMethod {
    fn name(&self) -> &str;
    fn share(&self, draft: &Draft) -> Result<(), ShareError>;
}
