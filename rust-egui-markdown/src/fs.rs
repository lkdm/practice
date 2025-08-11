use dirs::download_dir;
use rfd::FileDialog;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};
use thiserror::Error;
use uuid::Uuid;

use crate::doc::{Document, DocumentId, DocumentRepository};

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("I/O error occurred: {0}")]
    IO(#[from] std::io::Error),

    #[error("File not found")]
    NotFound,

    #[error("Invalid file format")]
    InvalidFormat,
}

struct FileSystem {
    /// A mapping of entity IDs to PathBufs so that we can abstract away the file system
    registry: BTreeMap<Uuid, PathBuf>,
}

impl FileSystem {
    /// `reader` - returns a `BufReader` for reading file
    fn reader(&self, id: Uuid) -> Result<BufReader<File>, FileSystemError> {
        let path = self.registry.get(&id).ok_or(FileSystemError::NotFound)?;
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }

    /// `writer` - returns a `BufWriter` for writing to a file
    fn writer(&mut self, id: Uuid, data: &str) -> Result<BufWriter<File>, FileSystemError> {
        let path = self.registry.entry(id).or_insert_with(|| {
            PathBuf::from(format!("{}/{}.md", download_dir().unwrap().display(), id))
        });

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data.as_bytes())?;
        Ok(writer)
    }
}

impl DocumentRepository for FileSystem {
    fn save(&self, document: &Document) -> Result<(), String> {
        let id = document.id; // Assuming Document has an id field
        self.write(id.0, document).map_err(|e| e.to_string())
    }

    fn load(&self, id: DocumentId) -> Result<Document, String> {
        self.read(id.0).map_err(|e| e.to_string()) // Assuming DocumentId can be dereferenced to Uuid
    }
}
