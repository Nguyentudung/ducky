use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

use crate::core::error::{AppError, AppResult};

/// Uses the same stable app-data location and `ducky.db` filename as the prior Tauri app.
pub fn open_database() -> AppResult<Connection> {
    let dirs = ProjectDirs::from("com", "Ducky", "Ducky").ok_or_else(|| {
        AppError::Validation("Unable to determine the application data directory".into())
    })?;
    fs::create_dir_all(dirs.data_dir())
        .map_err(|e| AppError::filesystem("Unable to create the application data directory", e))?;
    Ok(Connection::open(dirs.data_dir().join("ducky.db"))?)
}
