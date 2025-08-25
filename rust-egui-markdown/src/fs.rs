use async_trait::async_trait;
use rfd::FileDialog;
use std::{
    collections::BTreeMap,
    fs::File,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use thiserror::Error;

use crate::doc::{Document, DocumentId};

#[derive(Clone)]
pub struct CreateDocumentRequest {
    id: DocumentId,
    title: String,
    text: String,
}
#[derive(Debug, Error)]
#[error("could not create document: {0}")]
pub struct CreateDocumentError(String);

/// DocumentRepository
///
/// The repository pattern was chosen because it keeps PathBufs (an implementation detail for the
/// file system) out of the core logic of the application.
#[async_trait]
pub trait DocumentRepository: Clone + Send + Sync + 'static {
    async fn create_document(&self, req: &CreateDocumentRequest)
    -> Result<(), CreateDocumentError>;
    // async fn update_document(
    //     &self,
    //     req: &UpdateDocumentRequest,
    // ) -> Result<Document, UpdateDocumentError>;
    //
    // /// Given an ID, returns a useable Document struct
    // ///
    // /// On desktop, this loads the file into memory
    // async fn get_document(&self, req: &GetDocumentRequest) -> Result<Document, GetDocumentError>;
    //
    // /// List Documents
    // ///
    // /// On desktop, this opens an rfd window, in which the user can select one or more paths. These
    // /// paths are added to the registry, and their Uuid handles are returned in a vector. Usually
    // /// the user will only select one. On desktop, `get_document` is called immediately on each of
    // /// these items.
    // async fn list_documents(
    //     &self,
    //     req: &ListDocumentsRequest,
    // ) -> Result<Vec<DocumentId>, ListDocumentsError>;
}

#[derive(Clone)]
/// FileSystem
///
/// Abstraction over the file sytsem
pub struct FileSystem {
    /// The FileSystem stores a list of Uuids mapped to system paths
    paths: Arc<RwLock<BTreeMap<DocumentId, PathBuf>>>,
    /// The default directory to open in an rfd window
    default_dir: Option<PathBuf>,
    /// Name of rfd filter
    filter_name: String,
    /// Name of extensions to filter by
    extensions: Vec<String>,
}

#[derive(Debug, Error)]
#[error("user cancelled")]
struct UserCancelled;

#[async_trait]
impl DocumentRepository for FileSystem {
    async fn create_document(
        &self,
        req: &CreateDocumentRequest,
    ) -> Result<(), CreateDocumentError> {
        let file_path = FileDialog::new()
            .set_title("Save Markdown Document")
            .add_filter(&self.filter_name, &self.extensions) // e.g. ("Markdown", &["md", "markdown"])
            .set_directory(
                self.default_dir
                    .clone()
                    .unwrap_or_else(|| std::env::current_dir().unwrap()),
            )
            .save_file();

        let path = match file_path {
            Some(path) => path,
            None => return Ok(()),
        };

        {
            // Update internal map with the document id and path
            let mut paths = self.paths.write().unwrap();
            paths.insert(req.id.clone(), path.clone());
        }

        Ok(())
    }

    // async fn update_document(
    //     &self,
    //     req: &UpdateDocumentRequest,
    // ) -> Result<Document, UpdateDocumentError> {
    //     todo!()
    // }
    //
    // async fn get_document(&self, req: &GetDocumentRequest) -> Result<Document, GetDocumentError> {
    //     todo!()
    // }
    //
    // async fn list_documents(
    //     &self,
    //     req: &ListDocumentsRequest,
    // ) -> Result<Vec<DocumentId>, ListDocumentsError> {
    //     todo!()
    // }
}

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Entry not found in registry")]
    NotFound,
}
//
// /// PersistenceHandler
// ///
// /// An abstracted interface over any method of persistence.
// pub trait PersistenceHandler<T> {
//     type Id: Into<Uuid>;
//
//     /// Loads and deserializes data associated with this ID.
//     ///
//     /// On FS: opens a file dialog to select and load a file.
//     fn get(&self, id: Self::Id) -> Result<T, PersistenceError>;
//
//     /// Creates and stores a new instance of `T`.
//     ///
//     /// On FS: opens a file dialog prompting user to save location,
//     /// writes data to a new file, and tracks it internally.
//     fn new(&mut self, id: Self::Id, value: T) -> Result<T, PersistenceError>;
//
//     /// Updates an existing persisted object.
//     ///
//     /// On FS: overwrites the file with updated contents.
//     fn update(&mut self, id: Self::Id, value: T) -> Result<T, PersistenceError>;
//
//     /// Deletes persisted object.
//     fn remove(&mut self, id: Self::Id) -> Result<(), PersistenceError>;
// }
//
// struct FileSystem {
//     paths: BTreeMap<Uuid, PathBuf>,
//     default_dir: Option<PathBuf>,
//     filter_name: String,
//     extensions: Vec<String>,
// }
//
// impl FileSystem {
//     fn new_dialogue(&self) -> rfd::FileDialog {
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
//         let extensions: Vec<&str> = self.extensions.iter().map(|s| s.as_str()).collect();
//
//         rfd::FileDialog::new()
//             .add_filter(&self.filter_name, &extensions)
//             .set_directory(default_directory)
//             .set_can_create_directories(true)
//     }
// }
//
// impl PersistenceHandler<Document> for FileSystem {
//     type Id = Uuid;
// }
//
// // impl FileSystem
// // where
// //     K: Into<Uuid> + Ord,
// // {
// //     pub fn new(default_dir: Option<PathBuf>) -> Self {
// //         Self {
// //             registry: DataRegistry::<K, PathBuf>::new(),
// //             default_dir,
// //         }
// //     }
// //
// //     pub fn registry(&self) -> &DataRegistry<K, PathBuf> {
// //         &self.registry
// //     }
// //
// //     /// `reader` - returns a `BufReader` for reading file
// //     fn reader(&self, path: PathBuf) -> Result<BufReader<File>, PersistenceError> {
// //         let file = File::open(path)?;
// //         Ok(BufReader::new(file))
// //     }
// //
// //     /// `writer` - returns a `BufWriter` for writing to a file
// //     fn writer(&mut self, path: PathBuf) -> Result<BufWriter<File>, PersistenceError> {
// //         let file = File::create(path)?;
// //         let mut writer = BufWriter::new(file);
// //         Ok(writer)
// //     }
// //
// //     fn rfd(&self) -> rfd::FileDialog {
// //         let default_directory = self
// //             .default_dir
// //             .clone()
// //             // If not, the downloads directory
// //             .unwrap_or_else(|| {
// //                 download_dir()
// //                     // Failing all else, just use the current directory
// //                     .unwrap_or_else(|| PathBuf::from("."))
// //             });
// //
// //         rfd::FileDialog::new()
// //             .add_filter("Markdown files", &[".md"])
// //             .set_directory(default_directory)
// //             .set_can_create_directories(true)
// //     }
// // }
// //
// // impl DocumentRepository for FileSystem<DocumentId> {
// //     fn save(&mut self, document: &Document) -> Result<(), PersistenceError> {
// //         let id = document.id();
// //
// //         // Get the file path from the file dialog; if None, return Ok(()) (no-op)
// //         let path = match self.rfd().save_file() {
// //             Some(p) => p,
// //             None => return Ok(()), // No file selected, exit gracefully
// //         };
// //
// //         // Write the document content to the selected file
// //         let mut writer = self.writer(path.clone())?;
// //         document.text().write_to(writer)?;
// //
// //         // Register the file path in the registry after successful write
// //         self.registry().insert(id.clone(), path);
// //
// //         Ok(())
// //     }
// //
// //     fn load(&self, id: DocumentId) -> Result<Document, PersistenceError> {
// //         self.read(id.0).map_err(|e| e.to_string()) // Assuming DocumentId can be dereferenced to Uuid
// //
// //         // let path = self.registry.get(&id)?;
// //     }
// //
// //     fn list(&self) -> Result<Vec<Document>, PersistenceError> {
// //         todo!()
// //     }
// // }
