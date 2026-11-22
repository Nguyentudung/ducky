use directories::ProjectDirs;
use rusqlite::{Connection, Result};
use std::fs;

pub fn open_database() -> Result<Connection> {
    let project_dirs = ProjectDirs::from("com", "Ducky", "Ducky")
        .expect("Unable to determine application data directory");

    let data_dir = project_dirs.data_dir();

    fs::create_dir_all(data_dir).expect("Unable to create application data directory");

    let db_path = data_dir.join("ducky.db");

    Connection::open(db_path)
}
