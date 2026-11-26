use rusqlite::{Connection, Result};

/// Additive, idempotent schema. It is byte-compatible with the Tauri 0.1 database.
pub fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS user_profile (
            id INTEGER PRIMARY KEY, name TEXT NOT NULL, avatar TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS problems (
            id INTEGER PRIMARY KEY, identifier TEXT NOT NULL DEFAULT '', title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '', platform TEXT NOT NULL DEFAULT '', url TEXT NOT NULL DEFAULT '',
            difficulty TEXT NOT NULL DEFAULT 'medium', tags TEXT NOT NULL DEFAULT '[]',
            status TEXT NOT NULL DEFAULT 'not_started', languages TEXT NOT NULL DEFAULT '[]',
            workspace_path TEXT NOT NULL, notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
            last_opened_at TEXT
        );
        CREATE TABLE IF NOT EXISTS recent_problems (
            problem_id INTEGER PRIMARY KEY REFERENCES problems(id) ON DELETE CASCADE,
            opened_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
        );
        CREATE UNIQUE INDEX IF NOT EXISTS idx_problems_identifier_unique ON problems(identifier) WHERE identifier != '';
        CREATE INDEX IF NOT EXISTS idx_problems_updated_at ON problems(updated_at DESC);
        CREATE INDEX IF NOT EXISTS idx_problems_status ON problems(status);
        CREATE INDEX IF NOT EXISTS idx_recent_problems_opened_at ON recent_problems(opened_at DESC);
        ",
    )
}
