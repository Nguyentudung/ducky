//! Application state and persistence boundary. Only this module owns the SQLite connection.

use crate::core::{
    database::{connection::open_database, schema::initialize_schema},
    error::AppResult,
    i18n::language::{AppLanguage, LanguageManager},
    workspace::ensure_workspace_root,
};
use directories::{ProjectDirs, UserDirs};
use rusqlite::Connection;
use std::{path::PathBuf, sync::Mutex};

pub struct AppState {
    pub db: Mutex<Connection>,
    pub language: Mutex<LanguageManager>,
    pub theme: Mutex<String>,
}

impl AppState {
    pub fn new() -> AppResult<Self> {
        let conn = open_database()?;
        initialize_schema(&conn)?;
        let workspace = ensure_workspace_root(
            &load_setting(&conn, "workspace_path")
                .map(PathBuf::from)
                .unwrap_or_else(default_workspace_path),
        )?;
        save_setting(&conn, "workspace_path", &workspace.to_string_lossy())?;
        let language = match load_setting(&conn, "language").as_deref() {
            Some("en") => AppLanguage::English,
            Some("vi") => AppLanguage::Vietnamese,
            _ => AppLanguage::System,
        };
        let theme = match load_setting(&conn, "theme").as_deref() {
            Some("light") => "light",
            Some("dark") => "dark",
            _ => "system",
        }
        .to_string();
        Ok(Self {
            db: Mutex::new(conn),
            language: Mutex::new(LanguageManager::new(language)),
            theme: Mutex::new(theme),
        })
    }
}

pub fn load_setting(conn: &Connection, key: &str) -> Option<String> {
    conn.query_row(
        "SELECT value FROM app_settings WHERE key=?1",
        [key],
        |row| row.get(0),
    )
    .ok()
}
pub fn save_setting(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
    conn.execute("INSERT INTO app_settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",[key,value])?;
    Ok(())
}
/// `DUCKY_WORKSPACE` is an opt-in absolute override for CI/headless QA; normal users retain
/// the platform Documents directory default.
pub fn default_workspace_path() -> PathBuf {
    std::env::var_os("DUCKY_WORKSPACE")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| UserDirs::new().and_then(|d| d.document_dir().map(PathBuf::from)))
        .or_else(|| ProjectDirs::from("com", "Ducky", "Ducky").map(|d| d.data_dir().to_path_buf()))
        .unwrap_or_else(std::env::temp_dir)
        .join("Ducky")
}
pub fn workspace_path(conn: &Connection) -> AppResult<PathBuf> {
    ensure_workspace_root(
        &load_setting(conn, "workspace_path")
            .map(PathBuf::from)
            .unwrap_or_else(default_workspace_path),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::database::schema::initialize_schema;
    #[test]
    fn settings_are_persisted_and_replaced_by_key() {
        let c = Connection::open_in_memory().unwrap();
        initialize_schema(&c).unwrap();
        save_setting(&c, "default_language", "rust").unwrap();
        save_setting(&c, "default_language", "python").unwrap();
        assert_eq!(
            load_setting(&c, "default_language").as_deref(),
            Some("python")
        );
    }
}
