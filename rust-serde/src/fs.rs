use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use bincode::config::Configuration;
use bincode::serde::Compat;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::service::{ServiceError, ServicePersistence, Services};

#[derive(Debug, Error)]
pub enum BinaryFileError {
    #[error("file error: {0}")]
    IO(#[from] std::io::Error),

    #[error("could not encode to binary file: {0}")]
    Encode(#[from] bincode::error::EncodeError),

    #[error("could not decode from binary file: {0}")]
    Decode(#[from] bincode::error::DecodeError),
}

pub type Result<T> = std::result::Result<T, BinaryFileError>;

#[derive(Clone, Debug)]
pub struct BinaryFile {
    path: PathBuf,
}

impl Default for BinaryFile {
    fn default() -> Self {
        Self::new(PathBuf::from("/tmp/minitol/services.bin"))
    }
}

impl BinaryFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn bincode_config(&self) -> Configuration {
        bincode::config::standard()
    }

    fn write<T>(&self, req: T) -> Result<()>
    where
        T: Serialize,
    {
        let path = &self.path;

        // Create ancestor directories if they doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let serde_compat = Compat(req);
        bincode::encode_into_std_write(serde_compat, &mut writer, self.bincode_config())?;

        Ok(())
    }

    fn read<T>(&self) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let path = &self.path;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let Compat(res): Compat<T> =
            bincode::decode_from_std_read::<Compat<T>, _, _>(&mut reader, self.bincode_config())?;
        Ok(res)
    }
}

impl ServicePersistence for BinaryFile {
    fn write_services(&self, req: &Services) -> std::result::Result<(), ServiceError> {
        Ok(self.write(req)?)
    }

    fn read_services(&self) -> std::result::Result<Services, ServiceError> {
        Ok(self.read()?)
    }
}
