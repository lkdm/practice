use core::fmt;
use std::str::FromStr;

use clap::{Parser, ValueEnum};
use serde::Deserialize;
use tracing::Level;
use url::Url;

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
    #[arg(long, env, default_value = "http://127.0.0.1:9080")]
    pub api_url: Url,
}
