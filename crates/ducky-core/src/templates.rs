use crate::error::{CoreError, CoreResult};
use crate::models::{CreateTemplateInput, Problem, Template, UpdateTemplateInput};
use crate::validation::{normalize_enum, normalize_slug_id, optional_text, required_text};
use sqlx::{Row, SqlitePool};
use std::time::{SystemTime, UNIX_EPOCH};

const SELECT_TPL: &str = "SELECT id, name, kind, language_id, content, is_default, is_builtin, created_at, updated_at FROM templates";

fn row_to_template(r: &sqlx::sqlite::SqliteRow) -> Template {
    Template {
        id: r.get(0),
        name: r.get(1),
        kind: r.get(2),
        language_id: r.get(3),
        content: r.get(4),
        is_default: r.get::<i64, _>(5) != 0,
        is_builtin: r.get::<i64, _>(6) != 0,
        created_at: r.get(7),
        updated_at: r.get(8),
    }
}

pub async fn list_templates(pool: &SqlitePool) -> CoreResult<Vec<Template>> {
    let rows = sqlx::query(&format!(
        "{SELECT_TPL} ORDER BY kind ASC, is_default DESC, is_builtin DESC, name ASC"
    ))
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(row_to_template).collect())
}

pub async fn get_template(pool: &SqlitePool, id: &str) -> CoreResult<Template> {
    let clean_id = normalize_slug_id(id, "Template ID")?;
    let row = sqlx::query(&format!("{SELECT_TPL} WHERE id = ?1"))
        .bind(&clean_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Template '{clean_id}' was not found")))?;
    Ok(row_to_template(&row))
}

pub async fn create_template(
    pool: &SqlitePool,
    input: CreateTemplateInput,
) -> CoreResult<Template> {
    let name = required_text(&input.name, "Template name", 80)?;
    let kind = normalize_enum(&input.kind, &["source", "readme"], "template kind")?;
    let content = optional_text(&input.content, "Template content", 100_000)?;

    let language_id = match (kind.as_str(), input.language_id) {
        ("source", Some(lang)) if !lang.trim().is_empty() => {
            let lang_id = normalize_slug_id(&lang, "Language ID")?;
            let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM languages WHERE id = ?1")
                .bind(&lang_id)
                .fetch_one(pool)
                .await?;
            if exists == 0 {
                return Err(CoreError::Validation(format!(
                    "Unknown language '{lang_id}'"
                )));
            }
            Some(lang_id)
        }
        ("source", _) => {
            return Err(CoreError::Validation(
                "Source templates require a language_id".into(),
            ));
        }
        ("readme", _) => None,
        _ => unreachable!(),
    };

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let slug_base: String = name
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .chars()
        .take(20)
        .collect();
    let id = if slug_base.is_empty() {
        format!("tpl-{nonce}")
    } else {
        format!("{slug_base}-{}", nonce % 100_000)
    };

    let mut tx = pool.begin().await?;
    if input.is_default {
        clear_default_for_scope(&mut tx, &kind, language_id.as_deref()).await?;
    }
    sqlx::query(
        "INSERT INTO templates (id, name, kind, language_id, content, is_default, is_builtin)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
    )
    .bind(&id)
    .bind(&name)
    .bind(&kind)
    .bind(&language_id)
    .bind(&content)
    .bind(if input.is_default { 1 } else { 0 })
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    get_template(pool, &id).await
}

pub async fn update_template(
    pool: &SqlitePool,
    id: &str,
    input: UpdateTemplateInput,
) -> CoreResult<Template> {
    let current = get_template(pool, id).await?;
    let name = match input.name {
        Some(v) => required_text(&v, "Template name", 80)?,
        None => current.name,
    };
    let content = match input.content {
        Some(v) => {
            if v.chars().count() > 100_000 {
                return Err(CoreError::Validation("Template content is too long".into()));
            }
            v
        }
        None => current.content,
    };
    let is_default = input.is_default.unwrap_or(current.is_default);

    let mut tx = pool.begin().await?;
    if is_default && !current.is_default {
        clear_default_for_scope(&mut tx, &current.kind, current.language_id.as_deref()).await?;
    }
    sqlx::query(
        "UPDATE templates
         SET name = ?1, content = ?2, is_default = ?3, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id = ?4",
    )
    .bind(&name)
    .bind(&content)
    .bind(if is_default { 1 } else { 0 })
    .bind(&current.id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    get_template(pool, &current.id).await
}

pub async fn delete_template(pool: &SqlitePool, id: &str) -> CoreResult<()> {
    let tpl = get_template(pool, id).await?;
    if tpl.is_builtin {
        return Err(CoreError::Validation(
            "Built-in templates cannot be deleted; you can edit their content instead".into(),
        ));
    }
    sqlx::query("DELETE FROM templates WHERE id = ?1")
        .bind(&tpl.id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn clear_default_for_scope(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    kind: &str,
    language_id: Option<&str>,
) -> CoreResult<()> {
    match language_id {
        Some(lang) => {
            sqlx::query("UPDATE templates SET is_default = 0 WHERE kind = ?1 AND language_id = ?2")
                .bind(kind)
                .bind(lang)
                .execute(&mut **tx)
                .await?;
        }
        None => {
            sqlx::query(
                "UPDATE templates SET is_default = 0 WHERE kind = ?1 AND language_id IS NULL",
            )
            .bind(kind)
            .execute(&mut **tx)
            .await?;
        }
    }
    Ok(())
}

pub async fn resolve_source_content(
    pool: &SqlitePool,
    language_id: &str,
    template_mode_or_id: &str,
) -> CoreResult<String> {
    let mode = template_mode_or_id.trim();
    if mode.eq_ignore_ascii_case("empty") {
        return Ok(String::new());
    }
    if !mode.is_empty() && !mode.eq_ignore_ascii_case("starter") {
        let tpl = get_template(pool, mode).await?;
        if tpl.kind != "source" {
            return Err(CoreError::Validation(
                "Selected template is not a source template".into(),
            ));
        }
        return Ok(tpl.content);
    }

    let row = sqlx::query(
        "SELECT content FROM templates
         WHERE kind = 'source' AND language_id = ?1
         ORDER BY is_default DESC, is_builtin DESC, updated_at DESC
         LIMIT 1",
    )
    .bind(language_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.get::<String, _>(0)).unwrap_or_default())
}

pub async fn resolve_readme_template(
    pool: &SqlitePool,
    template_id: Option<&str>,
) -> CoreResult<String> {
    if let Some(id) = template_id.map(str::trim).filter(|s| !s.is_empty()) {
        let tpl = get_template(pool, id).await?;
        if tpl.kind != "readme" {
            return Err(CoreError::Validation(
                "Selected template is not a README template".into(),
            ));
        }
        return Ok(tpl.content);
    }

    let row = sqlx::query(
        "SELECT content FROM templates
         WHERE kind = 'readme'
         ORDER BY is_default DESC, is_builtin DESC, updated_at DESC
         LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    Ok(row
        .map(|r| r.get::<String, _>(0))
        .unwrap_or_else(crate::samples::load_readme_template))
}

pub fn render_readme(template: &str, problem: &Problem) -> String {
    let tags = if problem.tags.is_empty() {
        "—".to_string()
    } else {
        problem.tags.join(", ")
    };
    let languages = if problem.languages.is_empty() {
        "—".to_string()
    } else {
        problem.languages.join(", ")
    };
    let platform = if problem.platform.is_empty() {
        "—"
    } else {
        problem.platform.as_str()
    };
    let url = if problem.url.is_empty() {
        "—"
    } else {
        problem.url.as_str()
    };
    let description = if problem.description.is_empty() {
        "No description provided."
    } else {
        problem.description.as_str()
    };
    let notes = if problem.notes.is_empty() {
        "Add your notes, edge cases, and complexity analysis here."
    } else {
        problem.notes.as_str()
    };

    template
        .replace("{{title}}", &problem.title)
        .replace("{{identifier}}", &problem.identifier)
        .replace("{{platform}}", platform)
        .replace("{{source}}", platform)
        .replace("{{url}}", url)
        .replace("{{difficulty}}", &problem.difficulty)
        .replace("{{status}}", &problem.status)
        .replace("{{tags}}", &tags)
        .replace("{{languages}}", &languages)
        .replace("{{description}}", description)
        .replace("{{statement}}", description)
        .replace("{{notes}}", notes)
        .replace("{{created_at}}", &problem.created_at)
        .replace("{{updated_at}}", &problem.updated_at)
}
