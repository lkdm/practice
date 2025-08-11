use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
    ops::Deref,
    path::PathBuf,
    rc::Rc,
};

use ropey::Rope;

const MAX_TITLE_LEN: usize = 50;

pub trait DocumentRepository {
    fn load(&self, id: DocumentId) -> Result<Document, String>;
    fn save(&self, document: &Document) -> Result<(), String>;
    fn list(&self) -> Result<Vec<Document>, String>;
}

/// Shareable reference to a [`Document`] with interior mutability
pub type SharedDocument = Rc<RefCell<Document>>;

/// DocumentId - References a document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DocumentId(pub u32);

impl Deref for DocumentId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        Self(0)
    }
}

/// The title of a document
pub struct Title(String);

impl Deref for Title {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

/// The state the file is in
#[derive(Debug, Clone)]
pub enum FileState {
    Saved(PathBuf),
    Modified(PathBuf),
    New,
}

impl FileState {
    //! Outlines legal moves; you cannot go from modified -> saved, for example

    /// Transition from New -> Saved
    pub fn save(self, path: PathBuf) -> FileState {
        FileState::Saved(path)
    }

    /// Transition from Saved -> Modified
    pub fn modify(self) -> Option<FileState> {
        match self {
            FileState::Saved(path) => Some(FileState::Modified(path)),
            _ => None, // New cannot go directly to Modified
        }
    }

    /// Transition from Modified -> Saved (after saving)
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

/// A Document owns the content, the file state, and any other metadata linked to it
pub struct Document {
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
        let file = std::fs::File::open(&path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader)?;
        // TODO: Parse metadata
        // TODO: Add error type for metadata parse failure
        Ok(Self {
            id,
            state: FileState::Saved(path),
            text: rope,
            metadata: Metadata::default(),
        })
    }

    pub fn update_text(&mut self, new_text: &str) {
        self.text = Rope::from_str(new_text);
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        match self.state {
            FileState::Saved(_) => {
                // If the document is already saved, we can simply return Ok
                Ok(())
            }
            FileState::Modified(ref path) => {
                // Open the file for writing
                let file = File::create(path)?; // Use create to overwrite the file
                let mut writer = BufWriter::new(file);

                // Write the content of the Rope to the file
                self.text.write_to(&mut writer)?;

                // Flush the writer to ensure all data is written
                writer.flush()?;

                // Update the state to Saved
                self.state = FileState::Saved(path.clone());

                Ok(())
            }
            FileState::New => {
                // Return an error indicating that the user should choose a path
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Document is new, please specify a save path.",
                ))
            }
        }
    }

    pub fn save_as(&mut self, new_path: PathBuf) -> Result<(), std::io::Error> {
        // Update the state to reflect the new path
        self.state = FileState::Modified(new_path.clone());

        // Call the save method to perform the actual saving
        self.save() // This will handle the writing to the new path
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new(DocumentId::default())
    }
}

pub struct DocumentManager {
    documents: BTreeMap<DocumentId, SharedDocument>,
    next_id: u32,
}

impl DocumentManager {
    pub fn new() -> Self {
        Self {
            documents: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub fn add_document(&mut self) -> SharedDocument {
        let id = DocumentId(self.next_id);
        let document = Rc::new(RefCell::new(Document::new(id)));
        self.documents.insert(id, document.clone());
        self.next_id += 1;
        document
    }

    pub fn remove_document(&mut self, id: DocumentId) {
        self.documents.remove(&id);
    }

    pub fn get_document(&self, id: DocumentId) -> Option<SharedDocument> {
        self.documents.get(&id).cloned() // Return a clone of the Rc
    }

    pub fn list_documents(&self) -> Vec<SharedDocument> {
        self.documents.values().cloned().collect()
    }
}

impl Default for DocumentManager {
    fn default() -> Self {
        Self::new()
    }
}
