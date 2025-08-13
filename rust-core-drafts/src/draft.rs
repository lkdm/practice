use ropey::Rope;
use thiserror::Error;
use uuid::Uuid;

use crate::error::Result;

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

trait ShareMethod {
    fn label(&self) -> &str;
    fn name(&self) -> &str;
    fn share(&self, draft: &Draft) -> Result<()>;
}
