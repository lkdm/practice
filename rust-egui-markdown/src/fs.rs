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

    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Entry not found in registry")]
    NotFound,
}

/// PersistenceHandler
///
/// An abstracted interface over any method of persistence.
pub trait PersistenceHandler<T> {
    type Id: Into<Uuid>;

    /// Loads and deserializes data associated with this ID.
    ///
    /// On FS: opens a file dialog to select and load a file.
    fn get(&self, id: Self::Id) -> Result<T, PersistenceError>;

    /// Creates and stores a new instance of `T`.
    ///
    /// On FS: opens a file dialog prompting user to save location,
    /// writes data to a new file, and tracks it internally.
    fn new(&mut self, id: Self::Id, value: T) -> Result<T, PersistenceError>;

    /// Updates an existing persisted object.
    ///
    /// On FS: overwrites the file with updated contents.
    fn update(&mut self, id: Self::Id, value: T) -> Result<T, PersistenceError>;

    /// Deletes persisted object.
    fn remove(&mut self, id: Self::Id) -> Result<(), PersistenceError>;
}

struct FileSystem {
    paths: BTreeMap<Uuid, PathBuf>,
    default_dir: Option<PathBuf>,
    filter_name: String,
    extensions: Vec<String>,
}

impl FileSystem {
    fn new_dialogue(&self) -> rfd::FileDialog {
        let default_directory = self
            .default_dir
            .clone()
            // If not, the downloads directory
            .unwrap_or_else(|| {
                download_dir()
                    // Failing all else, just use the current directory
                    .unwrap_or_else(|| PathBuf::from("."))
            });

        let extensions: Vec<&str> = self.extensions.iter().map(|s| s.as_str()).collect();

        rfd::FileDialog::new()
            .add_filter(&self.filter_name, &extensions)
            .set_directory(default_directory)
            .set_can_create_directories(true)
    }
}

impl PersistenceHandler<Document> for FileSystem {
    type Id = Uuid;
}

// impl FileSystem
// where
//     K: Into<Uuid> + Ord,
// {
//     pub fn new(default_dir: Option<PathBuf>) -> Self {
//         Self {
//             registry: DataRegistry::<K, PathBuf>::new(),
//             default_dir,
//         }
//     }
//
//     pub fn registry(&self) -> &DataRegistry<K, PathBuf> {
//         &self.registry
//     }
//
//     /// `reader` - returns a `BufReader` for reading file
//     fn reader(&self, path: PathBuf) -> Result<BufReader<File>, PersistenceError> {
//         let file = File::open(path)?;
//         Ok(BufReader::new(file))
//     }
//
//     /// `writer` - returns a `BufWriter` for writing to a file
//     fn writer(&mut self, path: PathBuf) -> Result<BufWriter<File>, PersistenceError> {
//         let file = File::create(path)?;
//         let mut writer = BufWriter::new(file);
//         Ok(writer)
//     }
//
//     fn rfd(&self) -> rfd::FileDialog {
//         let default_directory = self
//             .default_dir
//             .clone()
//             // If not, the downloads directory
//             .unwrap_or_else(|| {
//                 download_dir()
//                     // Failing all else, just use the current directory
//                     .unwrap_or_else(|| PathBuf::from("."))
//             });
//
//         rfd::FileDialog::new()
//             .add_filter("Markdown files", &[".md"])
//             .set_directory(default_directory)
//             .set_can_create_directories(true)
//     }
// }
//
// impl DocumentRepository for FileSystem<DocumentId> {
//     fn save(&mut self, document: &Document) -> Result<(), PersistenceError> {
//         let id = document.id();
//
//         // Get the file path from the file dialog; if None, return Ok(()) (no-op)
//         let path = match self.rfd().save_file() {
//             Some(p) => p,
//             None => return Ok(()), // No file selected, exit gracefully
//         };
//
//         // Write the document content to the selected file
//         let mut writer = self.writer(path.clone())?;
//         document.text().write_to(writer)?;
//
//         // Register the file path in the registry after successful write
//         self.registry().insert(id.clone(), path);
//
//         Ok(())
//     }
//
//     fn load(&self, id: DocumentId) -> Result<Document, PersistenceError> {
//         self.read(id.0).map_err(|e| e.to_string()) // Assuming DocumentId can be dereferenced to Uuid
//
//         // let path = self.registry.get(&id)?;
//     }
//
//     fn list(&self) -> Result<Vec<Document>, PersistenceError> {
//         todo!()
//     }
// }
