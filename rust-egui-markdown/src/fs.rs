use dirs::download_dir;
use rfd::FileDialog;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
    ops::Deref,
    path::{Path, PathBuf},
};
use thiserror::Error;
use uuid::Uuid;

use crate::doc::{Document, DocumentId, DocumentRepository};

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Registry error: {0}")]
    Registry(#[from] DataRegistryError),

    #[error("Invalid file format")]
    InvalidFormat,
}

#[derive(Debug, Error)]
pub enum DataRegistryError {
    #[error("File not found")]
    NotFound,
}

/// Maps unique Ids to storage handles
/// For example, a PathBuf or Url
struct DataRegistry<K, V>(BTreeMap<K, V>);

impl<K, V> DataRegistry<K, V>
where
    K: Into<Uuid> + Ord,
{
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn get(&self, id: &K) -> Result<&V, DataRegistryError> {
        // Use `self.0` to access the internal BTreeMap of the tuple struct.
        self.0.get(id).ok_or(DataRegistryError::NotFound)
    }

    fn insert(&mut self, id: K, value: V) {
        self.0.insert(id, value);
    }

    fn remove(&mut self, id: &K) {
        self.0.remove(id);
    }
}

struct FileSystem<K> {
    /// A mapping of entity IDs to PathBufs so that we can abstract away the file system
    registry: DataRegistry<K, PathBuf>,
    default_dir: Option<PathBuf>,
}

impl<K> FileSystem<K>
where
    K: Into<Uuid> + Ord,
{
    pub fn new(default_dir: Option<PathBuf>) -> Self {
        Self {
            registry: DataRegistry::<K, PathBuf>::new(),
            default_dir,
        }
    }

    pub fn registry(&self) -> &DataRegistry<K, PathBuf> {
        &self.registry
    }

    /// `reader` - returns a `BufReader` for reading file
    fn reader(&self, path: PathBuf) -> Result<BufReader<File>, PersistenceError> {
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }

    /// `writer` - returns a `BufWriter` for writing to a file
    fn writer(&mut self, path: PathBuf) -> Result<BufWriter<File>, PersistenceError> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        Ok(writer)
    }

    fn rfd(&self) -> rfd::FileDialog {
        let default_directory = self
            .default_dir
            .clone()
            // If not, the downloads directory
            .unwrap_or_else(|| {
                download_dir()
                    // Failing all else, just use the current directory
                    .unwrap_or_else(|| PathBuf::from("."))
            });

        rfd::FileDialog::new()
            .add_filter("Markdown files", &[".md"])
            .set_directory(default_directory)
            .set_can_create_directories(true)
    }
}

impl DocumentRepository for FileSystem<DocumentId> {
    fn save(&mut self, document: &Document) -> Result<(), PersistenceError> {
        let id = document.id();

        // Get the file path from the file dialog; if None, return Ok(()) (no-op)
        let path = match self.rfd().save_file() {
            Some(p) => p,
            None => return Ok(()), // No file selected, exit gracefully
        };

        // Write the document content to the selected file
        let mut writer = self.writer(path.clone())?;
        document.text().write_to(writer)?;

        // Register the file path in the registry after successful write
        self.registry().insert(id.clone(), path);

        Ok(())
    }

    fn load(&self, id: DocumentId) -> Result<Document, PersistenceError> {
        self.read(id.0).map_err(|e| e.to_string()) // Assuming DocumentId can be dereferenced to Uuid

        // let path = self.registry.get(&id)?;
    }

    fn list(&self) -> Result<Vec<Document>, PersistenceError> {
        todo!()
    }
}
