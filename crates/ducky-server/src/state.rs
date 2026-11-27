use sqlx::SqlitePool;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub default_workspace: PathBuf,
    pub host: String,
    pub port: u16,
    pub dev_origin: Option<String>,
    pub version: &'static str,
}
