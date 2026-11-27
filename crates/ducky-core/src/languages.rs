use crate::error::{CoreError, CoreResult};
use crate::models::{CreateLanguageInput, Language, UpdateLanguageInput};
use crate::validation::{normalize_slug_id, required_text, validate_single_component};
use sqlx::{Row, SqlitePool};

const SELECT_LANG: &str = "SELECT id, name, extension, default_filename, folder_name, is_builtin, enabled, sort_order, created_at, updated_at FROM languages";

fn row_to_language(r: &sqlx::sqlite::SqliteRow) -> Language {
    Language {
        id: r.get(0),
        name: r.get(1),
        extension: r.get(2),
        default_filename: r.get(3),
        folder_name: r.get(4),
        is_builtin: r.get::<i64, _>(5) != 0,
        enabled: r.get::<i64, _>(6) != 0,
        sort_order: r.get(7),
        created_at: r.get(8),
        updated_at: r.get(9),
    }
}

pub async fn list_languages(pool: &SqlitePool) -> CoreResult<Vec<Language>> {
    let rows = sqlx::query(&format!("{SELECT_LANG} ORDER BY sort_order ASC, name ASC"))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(row_to_language).collect())
}

pub async fn get_language(pool: &SqlitePool, id: &str) -> CoreResult<Language> {
    let normalized = normalize_slug_id(id, "Language ID")?;
    let row = sqlx::query(&format!("{SELECT_LANG} WHERE id = ?1"))
        .bind(&normalized)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Language '{normalized}' was not found")))?;
    Ok(row_to_language(&row))
}

pub async fn create_language(
    pool: &SqlitePool,
    input: CreateLanguageInput,
) -> CoreResult<Language> {
    let id = normalize_slug_id(&input.id, "Language ID")?;
    let name = required_text(&input.name, "Language name", 40)?;
    let extension = input
        .extension
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase();
    if extension.is_empty()
        || extension.len() > 16
        || !extension.chars().all(|c| c.is_ascii_alphanumeric())
    {
        return Err(CoreError::Validation(
            "File extension must be 1-16 alphanumeric characters".into(),
        ));
    }
    let default_filename = validate_single_component(&input.default_filename, "Default filename")?;
    let folder_name = validate_single_component(&input.folder_name, "Folder name")?;

    let existing: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM languages WHERE id = ?1")
        .bind(&id)
        .fetch_one(pool)
        .await?;
    if existing > 0 {
        return Err(CoreError::Conflict(format!(
            "A language with ID '{id}' already exists"
        )));
    }

    let max_order: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(sort_order), 0) FROM languages")
        .fetch_one(pool)
        .await?;

    sqlx::query(
        "INSERT INTO languages (id, name, extension, default_filename, folder_name, is_builtin, enabled, sort_order)
         VALUES (?1, ?2, ?3, ?4, ?5, 0, 1, ?6)",
    )
    .bind(&id)
    .bind(&name)
    .bind(&extension)
    .bind(&default_filename)
    .bind(&folder_name)
    .bind(max_order + 1)
    .execute(pool)
    .await?;

    get_language(pool, &id).await
}

pub async fn update_language(
    pool: &SqlitePool,
    id: &str,
    input: UpdateLanguageInput,
) -> CoreResult<Language> {
    let current = get_language(pool, id).await?;

    let name = match input.name {
        Some(v) => required_text(&v, "Language name", 40)?,
        None => current.name,
    };
    let extension = match input.extension {
        Some(v) => {
            let ext = v.trim().trim_start_matches('.').to_ascii_lowercase();
            if ext.is_empty() || ext.len() > 16 || !ext.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(CoreError::Validation(
                    "File extension must be 1-16 alphanumeric characters".into(),
                ));
            }
            ext
        }
        None => current.extension,
    };
    let default_filename = match input.default_filename {
        Some(v) => validate_single_component(&v, "Default filename")?,
        None => current.default_filename,
    };
    let folder_name = match input.folder_name {
        Some(v) => validate_single_component(&v, "Folder name")?,
        None => current.folder_name,
    };
    let enabled = input.enabled.unwrap_or(current.enabled);
    let sort_order = input.sort_order.unwrap_or(current.sort_order);

    sqlx::query(
        "UPDATE languages
         SET name = ?1, extension = ?2, default_filename = ?3, folder_name = ?4,
             enabled = ?5, sort_order = ?6, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id = ?7",
    )
    .bind(&name)
    .bind(&extension)
    .bind(&default_filename)
    .bind(&folder_name)
    .bind(if enabled { 1 } else { 0 })
    .bind(sort_order)
    .bind(&current.id)
    .execute(pool)
    .await?;

    get_language(pool, &current.id).await
}

pub async fn delete_language(pool: &SqlitePool, id: &str) -> CoreResult<()> {
    let lang = get_language(pool, id).await?;
    if lang.is_builtin {
        return Err(CoreError::Validation(
            "Built-in languages cannot be deleted; disable them instead".into(),
        ));
    }
    sqlx::query("DELETE FROM languages WHERE id = ?1")
        .bind(&lang.id)
        .execute(pool)
        .await?;
    Ok(())
}
