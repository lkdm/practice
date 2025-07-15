use clap::Parser;
use serde::Deserialize;

pub type Port = u16;

/// Config
///
/// Configuration for [`example-rust-webservice`].
///
/// # Usage
///
/// See a list of available commands
/// ```sh
/// cargo run -- --help
/// ```
#[derive(Parser, Debug, Deserialize, Clone)]
#[command(name = &"Example Rust web service")]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Designates HOST IP address for the API
    #[arg(long, env, default_value = &"127.0.0.1")]
    pub ip_addr: std::net::IpAddr,

    /// Designates HOST port for the API.
    #[arg(long, env, default_value_t = 9080, value_parser = clap::value_parser!(u16).range(1..))]
    pub http_port: Port,

    #[arg(long, env, default_value_t = 9443, value_parser = clap::value_parser!(u16).range(1..))]
    pub https_port: Port,
}
