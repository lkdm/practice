use test_daemon::{error::Result, fs::BinaryFile, service::Service};

/// The DaemonContext gets passed around to different parts of the application
#[derive(Debug)]
struct DaemonContext {
    services: Vec<Service>,
    filestore: BinaryFile,
}

impl DaemonContext {
    pub fn new(services: &[Service], binary_file: &BinaryFile) -> Self {
        Self {
            services: services.to_owned(),
            filestore: binary_file.to_owned(),
        }
    }

    pub fn save(&self) -> test_daemon::fs::Result<()> {
        Ok(self.filestore.write(self.services.clone())?)
    }

    pub fn load(&mut self) -> test_daemon::fs::Result<()> {
        self.services = self.filestore.read()?;
        Ok(())
    }
}

impl Default for DaemonContext {
    fn default() -> Self {
        Self::new(
            &[Service::new(1, "one"), Service::new(2, "two")],
            &BinaryFile::default(),
        )
    }
}

pub fn main() -> Result<()> {
    let mut ctx = DaemonContext::default();
    ctx.save()?;
    ctx.load()?;

    println!("{:?}", ctx);

    Ok(())
}
