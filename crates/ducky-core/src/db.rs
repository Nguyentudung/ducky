use crate::error::CoreResult;
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::{path::Path, str::FromStr};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn init_pool(db_path: &Path) -> CoreResult<SqlitePool> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            crate::error::CoreError::filesystem("Could not create database directory", e)
        })?;
    }
    let url = format!("sqlite://{}", db_path.to_string_lossy());
    init_pool_from_url(&url).await
}

pub async fn init_memory_pool() -> CoreResult<SqlitePool> {
    init_pool_from_url("sqlite::memory:").await
}

async fn init_pool_from_url(url: &str) -> CoreResult<SqlitePool> {
    let opts = SqliteConnectOptions::from_str(url)?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    migrate_legacy_if_needed(&pool).await?;
    MIGRATOR.run(&pool).await?;
    crate::samples::sync_builtin_templates(&pool).await?;
    Ok(pool)
}

/// Preserves user settings, languages, and problems if an older Ducky database (Tauri/Slint)
/// exists at the data path without `_sqlx_migrations`.
async fn migrate_legacy_if_needed(pool: &SqlitePool) -> CoreResult<()> {
    let has_sqlx: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='_sqlx_migrations'",
    )
    .fetch_one(pool)
    .await?;
    if has_sqlx > 0 {
        return Ok(());
    }

    let has_problems: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='problems'",
    )
    .fetch_one(pool)
    .await?;
    let has_settings: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='app_settings'",
    )
    .fetch_one(pool)
    .await?;

    if has_problems == 0 && has_settings == 0 {
        return Ok(());
    }

    // Read legacy data before running clean v1 migration.
    let mut legacy_settings: Vec<(String, String)> = Vec::new();
    if has_settings > 0 {
        let rows = sqlx::query("SELECT key, value FROM app_settings")
            .fetch_all(pool)
            .await?;
        for r in rows {
            legacy_settings.push((r.get(0), r.get(1)));
        }
    }

    let mut legacy_readme_template: Option<String> = None;
    for (k, v) in &legacy_settings {
        if k == "readme_template" && !v.trim().is_empty() {
            legacy_readme_template = Some(v.clone());
        }
    }

    let mut legacy_problems = Vec::new();
    if has_problems > 0 {
        let cols: Vec<String> = sqlx::query("PRAGMA table_info(problems)")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r| r.get::<String, _>(1))
            .collect();

        if cols.iter().any(|c| c == "status_key") {
            // Tauri-era schema
            let rows = sqlx::query(
                "SELECT id, title, folder_name, source, url, status_key, statement, notes, created_at, updated_at FROM problems",
            )
            .fetch_all(pool)
            .await?;
            for r in rows {
                legacy_problems.push((
                    r.get::<i64, _>(0),
                    String::new(),
                    r.get::<String, _>(1),
                    r.get::<String, _>(6),
                    r.get::<String, _>(3),
                    r.get::<String, _>(4),
                    "medium".to_string(),
                    "[]".to_string(),
                    r.get::<String, _>(5),
                    "[]".to_string(),
                    String::new(),
                    r.get::<String, _>(2),
                    r.get::<String, _>(7),
                    r.get::<String, _>(8),
                    r.get::<String, _>(9),
                    Option::<String>::None,
                ));
            }
        } else if cols.iter().any(|c| c == "identifier") {
            // Slint-era schema
            let rows = sqlx::query(
                "SELECT id, identifier, title, description, platform, url, difficulty, tags, status, languages, workspace_path, notes, created_at, updated_at, last_opened_at FROM problems",
            )
            .fetch_all(pool)
            .await?;
            for r in rows {
                let ws: String = r.get(10);
                let folder_name = Path::new(&ws)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                legacy_problems.push((
                    r.get(0),
                    r.get(1),
                    r.get(2),
                    r.get(3),
                    r.get(4),
                    r.get(5),
                    r.get(6),
                    r.get(7),
                    r.get(8),
                    r.get(9),
                    ws,
                    folder_name,
                    r.get(11),
                    r.get(12),
                    r.get(13),
                    r.get(14),
                ));
            }
        }
    }

    let has_recent: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='recent_problems'",
    )
    .fetch_one(pool)
    .await?;
    let mut legacy_recent: Vec<(i64, String)> = Vec::new();
    if has_recent > 0 {
        let rows = sqlx::query("SELECT problem_id, opened_at FROM recent_problems")
            .fetch_all(pool)
            .await?;
        for r in rows {
            legacy_recent.push((r.get(0), r.get(1)));
        }
    }

    // Rename legacy tables away cleanly inside a transaction, run v1 migration, then import preserved rows.
    let mut tx = pool.begin().await?;
    for table in [
        "recent_problems",
        "problem_languages",
        "problems",
        "language_definitions",
        "problem_statuses",
        "app_settings",
        "user_profile",
    ] {
        let exists: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1")
                .bind(table)
                .fetch_one(&mut *tx)
                .await?;
        if exists > 0 {
            let sql = format!("DROP TABLE IF EXISTS {table}");
            sqlx::query(&sql).execute(&mut *tx).await?;
        }
    }
    tx.commit().await?;

    MIGRATOR.run(pool).await?;

    let mut tx = pool.begin().await?;
    for (k, v) in legacy_settings {
        let mapped_key = match k.as_str() {
            "app_language" => "language",
            "readme_template" => continue,
            other => other,
        };
        sqlx::query(
            "INSERT INTO app_settings(key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        )
        .bind(mapped_key)
        .bind(v)
        .execute(&mut *tx)
        .await?;
    }

    if let Some(readme) = legacy_readme_template {
        sqlx::query("UPDATE templates SET content = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now', '+1 second') WHERE id = 'readme-default'")
            .bind(readme)
            .execute(&mut *tx)
            .await?;
    }

    for p in legacy_problems {
        sqlx::query(
            "INSERT INTO problems (id, identifier, title, description, platform, url, difficulty, tags, status, languages, workspace_path, folder_name, notes, created_at, updated_at, last_opened_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)",
        )
        .bind(p.0)
        .bind(p.1)
        .bind(p.2)
        .bind(p.3)
        .bind(p.4)
        .bind(p.5)
        .bind(p.6)
        .bind(p.7)
        .bind(p.8)
        .bind(p.9)
        .bind(p.10)
        .bind(p.11)
        .bind(p.12)
        .bind(p.13)
        .bind(p.14)
        .bind(p.15)
        .execute(&mut *tx)
        .await?;
    }
    for (problem_id, opened_at) in legacy_recent {
        sqlx::query("INSERT OR IGNORE INTO recent_problems(problem_id, opened_at) VALUES (?1, ?2)")
            .bind(problem_id)
            .bind(opened_at)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;

    Ok(())
}
