use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(name = "ducky", version, about = "Ducky — Make coding feel easier.")]
pub struct ServerConfig {
    /// Host address to bind (defaults to loopback 127.0.0.1)
    #[arg(long, env = "DUCKY_HOST", default_value = "127.0.0.1")]
    pub host: String,

    /// Port to listen on
    #[arg(short, long, env = "DUCKY_PORT", default_value_t = 8090)]
    pub port: u16,

    /// Do not automatically open the default web browser on startup
    #[arg(long, env = "DUCKY_NO_OPEN", default_value_t = false)]
    pub no_open: bool,

    /// Override application data directory (where ducky.db is stored)
    #[arg(long, env = "DUCKY_DATA_DIR")]
    pub data_dir: Option<PathBuf>,

    /// Override default workspace root directory
    #[arg(long, env = "DUCKY_WORKSPACE")]
    pub workspace: Option<PathBuf>,

    /// Optional allowed frontend dev origin (e.g. http://localhost:5173)
    #[arg(long, env = "DUCKY_DEV_ORIGIN")]
    pub dev_origin: Option<String>,
}

impl ServerConfig {
    pub fn is_loopback(&self) -> bool {
        matches!(self.host.as_str(), "127.0.0.1" | "::1" | "localhost")
    }
}
