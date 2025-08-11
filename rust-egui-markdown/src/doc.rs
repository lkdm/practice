use std::{cell::RefCell, path::PathBuf, rc::Rc};

use ropey::Rope;

const MAX_TITLE_LEN: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// TODO: Use uuid or some other thing?
pub struct DocumentId(pub u32);

impl Default for DocumentId {
    fn default() -> Self {
        Self(0)
    }
}

pub struct Title(String);

impl Title {
    pub fn new(title: &str) -> Self {
        let truncated: String = title.chars().take(MAX_TITLE_LEN).collect();
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

pub struct Metadata {
    title: Title,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: Title::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileState {
    Saved(PathBuf),
    Modified(PathBuf),
    New,
}

impl FileState {
    // Transition from New -> Saved
    pub fn save(self, path: PathBuf) -> FileState {
        FileState::Saved(path)
    }

    // Transition from Saved -> Modified
    pub fn modify(self) -> Option<FileState> {
        match self {
            FileState::Saved(path) => Some(FileState::Modified(path)),
            _ => None, // New cannot go directly to Modified
        }
    }

    // Transition from Modified -> Saved (after saving)
    pub fn save_modified(self) -> Option<FileState> {
        match self {
            FileState::Modified(path) => Some(FileState::Saved(path)),
            _ => None,
        }
    }
}

impl Default for FileState {
    fn default() -> Self {
        Self::New
    }
}

struct Document {
    id: DocumentId,
    state: FileState,
    text: Rope,
    metadata: Metadata,
}

impl Document {
    pub fn new(id: DocumentId) -> Self {
        Self {
            id,
            state: FileState::New,
            text: Rope::new(),
            metadata: Metadata::default(),
        }
    }

    pub fn open(id: DocumentId, path: PathBuf) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(&path)?;
        let rope = Rope::from_str(&content);
        // TODO: Parse metadata
        Ok(Self {
            id,
            state: FileState::Saved(path),
            text: rope,
            metadata: Metadata::default(),
        })
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new(DocumentId::default())
    }
}

type SharedDocument = Rc<RefCell<Document>>;
