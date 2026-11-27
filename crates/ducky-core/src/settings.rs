use crate::error::{CoreError, CoreResult};
use crate::models::{AppSettings, SettingsPatch};
use crate::validation::{
    normalize_app_language, normalize_difficulty, normalize_file_layout, normalize_slug_id,
    normalize_status, normalize_template_mode, normalize_theme,
};
use crate::workspace::ensure_workspace_root;
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub async fn get_settings(pool: &SqlitePool, default_workspace: &Path) -> CoreResult<AppSettings> {
    let rows = sqlx::query("SELECT key, value FROM app_settings")
        .fetch_all(pool)
        .await?;
    let mut map = HashMap::new();
    for r in rows {
        map.insert(r.get::<String, _>(0), r.get::<String, _>(1));
    }

    let workspace_path = map
        .get("workspace_path")
        .filter(|s| !s.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| default_workspace.to_string_lossy().into_owned());

    let theme = map
        .get("theme")
        .and_then(|v| normalize_theme(v).ok())
        .unwrap_or_else(|| "system".to_string());

    let language = map
        .get("language")
        .and_then(|v| normalize_app_language(v).ok())
        .unwrap_or_else(|| "system".to_string());

    let default_language = map
        .get("default_language")
        .and_then(|v| normalize_slug_id(v, "default_language").ok())
        .unwrap_or_else(|| "cpp".to_string());

    let default_difficulty = map
        .get("default_difficulty")
        .and_then(|v| normalize_difficulty(v).ok())
        .unwrap_or_else(|| "medium".to_string());

    let default_status = map
        .get("default_status")
        .and_then(|v| normalize_status(v).ok())
        .unwrap_or_else(|| "not_started".to_string());

    let default_template_mode = map
        .get("default_template_mode")
        .and_then(|v| normalize_template_mode(v).ok())
        .unwrap_or_else(|| "starter".to_string());

    let group_by_platform = map
        .get("group_by_platform")
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);

    let file_layout = map
        .get("file_layout")
        .and_then(|v| normalize_file_layout(v).ok())
        .unwrap_or_else(|| "auto".to_string());

    let generate_readme = map
        .get("generate_readme")
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);

    Ok(AppSettings {
        workspace_path,
        theme,
        language,
        default_language,
        default_difficulty,
        default_status,
        default_template_mode,
        group_by_platform,
        file_layout,
        generate_readme,
    })
}

pub async fn patch_settings(
    pool: &SqlitePool,
    default_workspace: &Path,
    patch: SettingsPatch,
) -> CoreResult<AppSettings> {
    let mut updates: Vec<(&'static str, String)> = Vec::new();

    if let Some(ws) = patch.workspace_path {
        let canonical = ensure_workspace_root(Path::new(ws.trim()))?;
        updates.push(("workspace_path", canonical.to_string_lossy().into_owned()));
    }
    if let Some(theme) = patch.theme {
        updates.push(("theme", normalize_theme(&theme)?));
    }
    if let Some(lang) = patch.language {
        updates.push(("language", normalize_app_language(&lang)?));
    }
    if let Some(default_lang) = patch.default_language {
        let id = normalize_slug_id(&default_lang, "Default language")?;
        let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM languages WHERE id = ?1")
            .bind(&id)
            .fetch_one(pool)
            .await?;
        if exists == 0 {
            return Err(CoreError::Validation(format!(
                "Unknown default language '{id}'"
            )));
        }
        updates.push(("default_language", id));
    }
    if let Some(diff) = patch.default_difficulty {
        updates.push(("default_difficulty", normalize_difficulty(&diff)?));
    }
    if let Some(status) = patch.default_status {
        updates.push(("default_status", normalize_status(&status)?));
    }
    if let Some(mode) = patch.default_template_mode {
        updates.push(("default_template_mode", normalize_template_mode(&mode)?));
    }
    if let Some(group) = patch.group_by_platform {
        updates.push((
            "group_by_platform",
            if group { "true" } else { "false" }.to_string(),
        ));
    }
    if let Some(layout) = patch.file_layout {
        updates.push(("file_layout", normalize_file_layout(&layout)?));
    }
    if let Some(readme) = patch.generate_readme {
        updates.push((
            "generate_readme",
            if readme { "true" } else { "false" }.to_string(),
        ));
    }

    let mut tx = pool.begin().await?;
    for (k, v) in updates {
        sqlx::query(
            "INSERT INTO app_settings(key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        )
        .bind(k)
        .bind(v)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    get_settings(pool, default_workspace).await
}

pub async fn ensure_workspace_initialized(
    pool: &SqlitePool,
    default_workspace: &Path,
) -> CoreResult<PathBuf> {
    let settings = get_settings(pool, default_workspace).await?;
    let path = PathBuf::from(&settings.workspace_path);
    match ensure_workspace_root(&path) {
        Ok(canonical) => {
            if canonical.to_string_lossy() != settings.workspace_path {
                sqlx::query(
                    "INSERT INTO app_settings(key, value) VALUES ('workspace_path', ?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                )
                .bind(canonical.to_string_lossy().as_ref())
                .execute(pool)
                .await?;
            }
            Ok(canonical)
        }
        Err(_) => {
            // Do not crash if a previously configured external mount is temporarily unavailable;
            // still allow app startup and surface validation warning in Workspace API/UI.
            Ok(path)
        }
    }
}
