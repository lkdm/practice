pub type Promise<R> = Box<dyn Future<Output = R> + Send>;

pub mod error;
pub mod fs;
pub mod service;
