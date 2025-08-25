use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
    ops::Deref,
    path::PathBuf,
    rc::Rc,
};

use bon::Builder;
use ropey::Rope;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A Document owns the content, the file state, and any other metadata linked to it
#[derive(Builder, Clone)]
pub struct Document {
    #[builder(default, into)]
    id: DocumentId,
    #[builder(default, into)]
    title: Title,
    #[builder(default, into)]
    text: Rope,
    #[builder(default = SavedState::New)]
    state: SavedState,
}

/// DocumentId - References a document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DocumentId(pub Uuid);

impl Deref for DocumentId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Uuid> for DocumentId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Title(String);

impl Deref for Title {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Title {
    pub fn new(title: &str) -> Self {
        let truncated: String = title.chars().take(50).collect();
        Self(truncated)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self("Untitled".to_string())
    }
}

/// The state the file is in
#[derive(Debug, Clone)]
pub enum SavedState {
    New,
    NoChanges,
    Changed,
}
