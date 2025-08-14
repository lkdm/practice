use std::path::PathBuf;

use test_daemon::{error::Result, fs::BinaryFile, service::Services};

struct DaemonContext {
    services: Services,
    binary_file: BinaryFile,
}

impl DaemonContext {
    pub fn new(services: &Services, binary_file: &BinaryFile) -> Self {
        Self {
            services: services.to_owned(),
            binary_file: binary_file.to_owned(),
        }
    }
    pub fn write_services() {
        self.binary_file.
    }
}

impl Default for DaemonContext {
    fn default() -> Self {
        Self::new(&Services::default(), &BinaryFile::default())
    }
}

pub fn main() -> Result<()> {
    let ctx = DaemonContext::default();
    ctx.binary_file.write_services();
    Ok(())
}
