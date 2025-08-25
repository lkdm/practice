use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use thiserror::Error;

/// File error
///
/// Can be returned while writing to or reading from a text file
#[derive(Debug, Error)]
pub enum LocalFileError {
    #[error("file error: {0}")]
    IO(#[from] std::io::Error),
}

/// Helper type to slightly shorten method signatures
pub type Result<T> = std::result::Result<T, LocalFileError>;

/// File
///
/// A text file used to persist state on the filesystem
#[derive(Clone, Debug)]
pub struct LocalFile {
    path: PathBuf,
}

impl Default for LocalFile {
    fn default() -> Self {
        Self::new(PathBuf::from(""))
    }
}

impl LocalFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Write any type as plain text
    pub fn write<T>(&self, data: T) -> Result<()>
    where
        T: ToString,
    {
        let path = &self.path;

        // Create ancestor directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(data.to_string().as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    /// Read the file into a `String` (caller can parse further if needed)
    pub fn read(&self) -> Result<String> {
        let path = &self.path;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
