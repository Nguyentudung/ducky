use crate::error::{CoreError, CoreResult};
use crate::languages::{get_language, list_languages};
use crate::models::{
    AddProblemFileInput, CreateProblemInput, DashboardStats, GenerateReadmeInput, Problem,
    ProblemDetail, ProblemQuery, UpdateProblemInput, WorkspaceFile,
};
use crate::settings::get_settings;
use crate::templates::{render_readme, resolve_readme_template, resolve_source_content};
use crate::validation::{
    normalize_difficulty, normalize_file_layout, normalize_identifier, normalize_list,
    normalize_slug_id, normalize_status, normalize_url, optional_text, required_text,
    validate_single_component,
};
use crate::workspace::{
    CreateWorkspaceParams, PlannedSourceFile, add_language_file_to_workspace,
    create_problem_workspace, delete_problem_directory, list_problem_files, write_readme_file,
};
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

const SELECT_PROBLEM: &str = "SELECT id, identifier, title, description, platform, url, difficulty, tags, status, languages, workspace_path, folder_name, notes, created_at, updated_at, last_opened_at FROM problems";

fn row_to_problem(r: &sqlx::sqlite::SqliteRow) -> CoreResult<Problem> {
    let tags_raw: String = r.get(7);
    let langs_raw: String = r.get(9);
    Ok(Problem {
        id: r.get(0),
        identifier: r.get(1),
        title: r.get(2),
        description: r.get(3),
        platform: r.get(4),
        url: r.get(5),
        difficulty: r.get(6),
        tags: serde_json::from_str(&tags_raw).unwrap_or_default(),
        status: r.get(8),
        languages: serde_json::from_str(&langs_raw).unwrap_or_default(),
        workspace_path: r.get(10),
        folder_name: r.get(11),
        notes: r.get(12),
        created_at: r.get(13),
        updated_at: r.get(14),
        last_opened_at: r.get(15),
    })
}

pub async fn create_problem(
    pool: &SqlitePool,
    default_workspace: &Path,
    input: CreateProblemInput,
) -> CoreResult<ProblemDetail> {
    let settings = get_settings(pool, default_workspace).await?;

    let title = required_text(&input.title, "Problem title", 160)?;
    let identifier = normalize_identifier(&input.identifier)?;
    let description = optional_text(&input.description, "Description", 20_000)?;
    let platform = optional_text(&input.platform, "Platform", 80)?;
    let url = normalize_url(&input.url)?;
    let difficulty = if input.difficulty.trim().is_empty() {
        settings.default_difficulty.clone()
    } else {
        normalize_difficulty(&input.difficulty)?
    };
    let status = if input.status.trim().is_empty() {
        settings.default_status.clone()
    } else {
        normalize_status(&input.status)?
    };
    let tags = normalize_list(input.tags, "tag", 20, 40)?;
    let notes = optional_text(&input.notes, "Notes", 50_000)?;

    let raw_langs = if input.languages.is_empty() {
        vec![settings.default_language.clone()]
    } else {
        input.languages
    };
    let language_ids = normalize_list(raw_langs, "language", 10, 40)?;
    if language_ids.is_empty() {
        return Err(CoreError::Validation(
            "Select at least one programming language".into(),
        ));
    }

    ensure_identifier_available(pool, &identifier, None).await?;

    let all_languages = list_languages(pool).await?;
    let template_mode = input
        .template_mode
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(&settings.default_template_mode);

    let mut planned_files = Vec::new();
    let mut normalized_lang_ids = Vec::new();
    for raw_lang in &language_ids {
        let lang_id = normalize_slug_id(raw_lang, "Language")?;
        let lang = all_languages
            .iter()
            .find(|l| l.id == lang_id)
            .cloned()
            .ok_or_else(|| {
                CoreError::Validation(format!("Unsupported programming language: {lang_id}"))
            })?;

        let filename = match input.custom_filenames.get(&lang.id) {
            Some(custom) if !custom.trim().is_empty() => {
                validate_single_component(custom, "Custom filename")?
            }
            _ => lang.default_filename.clone(),
        };
        let content = resolve_source_content(pool, &lang.id, template_mode).await?;
        normalized_lang_ids.push(lang.id.clone());
        planned_files.push(PlannedSourceFile {
            language: lang,
            filename,
            content,
        });
    }

    let workspace_root = input
        .workspace_root
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(&settings.workspace_path));

    let group_by_platform = input
        .group_by_platform
        .unwrap_or(settings.group_by_platform);
    let file_layout = match input
        .file_layout
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(l) => normalize_file_layout(l)?,
        None => settings.file_layout.clone(),
    };
    let generate_readme = input.generate_readme.unwrap_or(settings.generate_readme);

    let draft_problem = Problem {
        id: 0,
        identifier: identifier.clone(),
        title: title.clone(),
        description: description.clone(),
        platform: platform.clone(),
        url: url.clone(),
        difficulty: difficulty.clone(),
        tags: tags.clone(),
        status: status.clone(),
        languages: normalized_lang_ids.clone(),
        workspace_path: String::new(),
        folder_name: String::new(),
        notes: notes.clone(),
        created_at: String::new(),
        updated_at: String::new(),
        last_opened_at: None,
    };

    let readme_rendered = if generate_readme {
        let tpl = resolve_readme_template(pool, None).await?;
        Some(render_readme(&tpl, &draft_problem))
    } else {
        None
    };

    let (problem_dir, folder_name) = create_problem_workspace(CreateWorkspaceParams {
        workspace_root: &workspace_root,
        title: &title,
        platform: &platform,
        custom_folder_name: input.folder_name.as_deref(),
        group_by_platform,
        file_layout: &file_layout,
        files: &planned_files,
        readme_content: readme_rendered.as_deref(),
    })?;

    let tags_json = serde_json::to_string(&tags)?;
    let langs_json = serde_json::to_string(&normalized_lang_ids)?;
    let ws_str = problem_dir.to_string_lossy().into_owned();

    let insert_res = sqlx::query(
        "INSERT INTO problems (identifier, title, description, platform, url, difficulty, tags, status, languages, workspace_path, folder_name, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
    )
    .bind(&identifier)
    .bind(&title)
    .bind(&description)
    .bind(&platform)
    .bind(&url)
    .bind(&difficulty)
    .bind(&tags_json)
    .bind(&status)
    .bind(&langs_json)
    .bind(&ws_str)
    .bind(&folder_name)
    .bind(&notes)
    .execute(pool)
    .await;

    let id = match insert_res {
        Ok(res) => res.last_insert_rowid(),
        Err(err) => {
            // Roll back freshly created workspace folder on DB failure.
            let _ = std::fs::remove_dir_all(&problem_dir);
            return Err(err.into());
        }
    };

    get_problem_detail(pool, id, false).await
}

pub async fn get_problem(pool: &SqlitePool, id: i64) -> CoreResult<Problem> {
    let row = sqlx::query(&format!("{SELECT_PROBLEM} WHERE id = ?1"))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::NotFound("Problem was not found".into()))?;
    row_to_problem(&row)
}

pub async fn get_problem_detail(
    pool: &SqlitePool,
    id: i64,
    record_open: bool,
) -> CoreResult<ProblemDetail> {
    if record_open {
        let _ = get_problem(pool, id).await?;
        let mut tx = pool.begin().await?;
        sqlx::query(
            "UPDATE problems SET last_opened_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            "INSERT INTO recent_problems(problem_id, opened_at)
             VALUES (?1, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
             ON CONFLICT(problem_id) DO UPDATE SET opened_at = excluded.opened_at",
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
    }

    let problem = get_problem(pool, id).await?;
    let languages = list_languages(pool).await?;
    let dir = Path::new(&problem.workspace_path);
    let workspace_exists = !problem.workspace_path.is_empty() && dir.is_dir();
    let files = if workspace_exists {
        list_problem_files(dir, &languages)
    } else {
        Vec::new()
    };

    Ok(ProblemDetail {
        problem,
        files,
        workspace_exists,
    })
}

pub async fn list_problems(pool: &SqlitePool, query: ProblemQuery) -> CoreResult<Vec<Problem>> {
    let mut qb = QueryBuilder::<Sqlite>::new(SELECT_PROBLEM);
    let mut has_where = false;

    let push_clause = |qb: &mut QueryBuilder<'_, Sqlite>, has_where: &mut bool| {
        if *has_where {
            qb.push(" AND ");
        } else {
            qb.push(" WHERE ");
            *has_where = true;
        }
    };

    let search = query.search.trim();
    if !search.is_empty() {
        push_clause(&mut qb, &mut has_where);
        let pat = format!("%{search}%");
        qb.push("(title LIKE ")
            .push_bind(pat.clone())
            .push(" OR identifier LIKE ")
            .push_bind(pat.clone())
            .push(" OR platform LIKE ")
            .push_bind(pat.clone())
            .push(" OR tags LIKE ")
            .push_bind(pat)
            .push(")");
    }

    if !query.status.trim().is_empty() {
        let status = normalize_status(&query.status)?;
        push_clause(&mut qb, &mut has_where);
        qb.push("status = ").push_bind(status);
    }

    if !query.difficulty.trim().is_empty() {
        let diff = normalize_difficulty(&query.difficulty)?;
        push_clause(&mut qb, &mut has_where);
        qb.push("difficulty = ").push_bind(diff);
    }

    if !query.platform.trim().is_empty() {
        push_clause(&mut qb, &mut has_where);
        qb.push("LOWER(platform) = LOWER(")
            .push_bind(query.platform.trim().to_string())
            .push(")");
    }

    if !query.language.trim().is_empty() {
        let lang = query.language.trim().to_string();
        push_clause(&mut qb, &mut has_where);
        qb.push("languages LIKE ")
            .push_bind(format!("%\"{lang}\"%"));
    }

    if !query.tag.trim().is_empty() {
        let tag = query.tag.trim().to_string();
        push_clause(&mut qb, &mut has_where);
        qb.push("tags LIKE ").push_bind(format!("%\"{tag}\"%"));
    }

    let sort_expr = match query.sort.trim().to_ascii_lowercase().as_str() {
        "title" => "title COLLATE NOCASE",
        "created" => "created_at",
        "difficulty" => {
            "CASE difficulty WHEN 'easy' THEN 1 WHEN 'medium' THEN 2 WHEN 'hard' THEN 3 ELSE 4 END"
        }
        "status" => {
            "CASE status WHEN 'not_started' THEN 1 WHEN 'in_progress' THEN 2 WHEN 'review' THEN 3 ELSE 4 END"
        }
        _ => "updated_at",
    };
    let dir = if query.dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

    qb.push(format!(" ORDER BY {sort_expr} {dir}, id DESC"));

    let rows = qb.build().fetch_all(pool).await?;
    rows.iter().map(row_to_problem).collect()
}

pub async fn update_problem(
    pool: &SqlitePool,
    id: i64,
    input: UpdateProblemInput,
) -> CoreResult<Problem> {
    let current = get_problem(pool, id).await?;

    let identifier = match input.identifier {
        Some(v) => normalize_identifier(&v)?,
        None => current.identifier,
    };
    ensure_identifier_available(pool, &identifier, Some(id)).await?;

    let title = match input.title {
        Some(v) => required_text(&v, "Problem title", 160)?,
        None => current.title,
    };
    let description = match input.description {
        Some(v) => optional_text(&v, "Description", 20_000)?,
        None => current.description,
    };
    let platform = match input.platform {
        Some(v) => optional_text(&v, "Platform", 80)?,
        None => current.platform,
    };
    let url = match input.url {
        Some(v) => normalize_url(&v)?,
        None => current.url,
    };
    let difficulty = match input.difficulty {
        Some(v) => normalize_difficulty(&v)?,
        None => current.difficulty,
    };
    let status = match input.status {
        Some(v) => normalize_status(&v)?,
        None => current.status,
    };
    let tags = match input.tags {
        Some(v) => normalize_list(v, "tag", 20, 40)?,
        None => current.tags,
    };
    let languages = match input.languages {
        Some(v) => {
            let list = normalize_list(v, "language", 10, 40)?;
            if list.is_empty() {
                return Err(CoreError::Validation(
                    "Select at least one programming language".into(),
                ));
            }
            list
        }
        None => current.languages,
    };
    let notes = match input.notes {
        Some(v) => optional_text(&v, "Notes", 50_000)?,
        None => current.notes,
    };

    sqlx::query(
        "UPDATE problems
         SET identifier = ?1, title = ?2, description = ?3, platform = ?4, url = ?5,
             difficulty = ?6, tags = ?7, status = ?8, languages = ?9, notes = ?10,
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id = ?11",
    )
    .bind(&identifier)
    .bind(&title)
    .bind(&description)
    .bind(&platform)
    .bind(&url)
    .bind(&difficulty)
    .bind(serde_json::to_string(&tags)?)
    .bind(&status)
    .bind(serde_json::to_string(&languages)?)
    .bind(&notes)
    .bind(id)
    .execute(pool)
    .await?;

    get_problem(pool, id).await
}

pub async fn delete_problem(
    pool: &SqlitePool,
    default_workspace: &Path,
    id: i64,
    delete_files: bool,
) -> CoreResult<()> {
    let problem = get_problem(pool, id).await?;
    if delete_files && !problem.workspace_path.is_empty() {
        let settings = get_settings(pool, default_workspace).await?;
        let ws_root = Path::new(&settings.workspace_path);
        delete_problem_directory(ws_root, Path::new(&problem.workspace_path))?;
    }

    let result = sqlx::query("DELETE FROM problems WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(CoreError::NotFound("Problem was not found".into()));
    }
    Ok(())
}

pub async fn add_problem_file(
    pool: &SqlitePool,
    default_workspace: &Path,
    id: i64,
    input: AddProblemFileInput,
) -> CoreResult<WorkspaceFile> {
    let mut problem = get_problem(pool, id).await?;
    if problem.workspace_path.is_empty() {
        return Err(CoreError::Validation(
            "This problem does not have a workspace folder configured".into(),
        ));
    }

    let settings = get_settings(pool, default_workspace).await?;
    let lang = get_language(pool, &input.language_id).await?;

    let tpl_mode = input
        .template_id
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(&settings.default_template_mode);
    let content = resolve_source_content(pool, &lang.id, tpl_mode).await?;

    let use_subdir = input
        .subdir
        .unwrap_or_else(|| match settings.file_layout.as_str() {
            "subdirs" => true,
            "flat" => false,
            _ => !problem.languages.is_empty() && !problem.languages.contains(&lang.id),
        });

    let created = add_language_file_to_workspace(
        Path::new(&problem.workspace_path),
        &lang,
        input.filename.as_deref(),
        &content,
        use_subdir,
    )?;

    if !problem.languages.contains(&lang.id) {
        problem.languages.push(lang.id.clone());
        sqlx::query(
            "UPDATE problems SET languages = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?2",
        )
        .bind(serde_json::to_string(&problem.languages)?)
        .bind(id)
        .execute(pool)
        .await?;
    }

    Ok(created)
}

pub async fn generate_problem_readme(
    pool: &SqlitePool,
    id: i64,
    input: GenerateReadmeInput,
) -> CoreResult<()> {
    let problem = get_problem(pool, id).await?;
    if problem.workspace_path.is_empty() {
        return Err(CoreError::Validation(
            "This problem does not have a workspace folder configured".into(),
        ));
    }
    let tpl = resolve_readme_template(pool, input.template_id.as_deref()).await?;
    let rendered = render_readme(&tpl, &problem);
    write_readme_file(
        Path::new(&problem.workspace_path),
        &rendered,
        input.overwrite,
    )
}

pub async fn dashboard_stats(pool: &SqlitePool) -> CoreResult<DashboardStats> {
    let counts = sqlx::query(
        "SELECT
            COUNT(*),
            COALESCE(SUM(CASE WHEN status = 'solved' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'in_progress' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'not_started' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN status = 'review' THEN 1 ELSE 0 END), 0)
         FROM problems",
    )
    .fetch_one(pool)
    .await?;

    let total: i64 = counts.get(0);
    let solved: i64 = counts.get(1);
    let in_progress: i64 = counts.get(2);
    let not_started: i64 = counts.get(3);
    let review: i64 = counts.get(4);
    let completion_percent = if total > 0 {
        ((solved as f64) / (total as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };

    let recent_rows = sqlx::query(
        "SELECT p.id, p.identifier, p.title, p.description, p.platform, p.url, p.difficulty, p.tags,
                p.status, p.languages, p.workspace_path, p.folder_name, p.notes, p.created_at, p.updated_at, p.last_opened_at
         FROM recent_problems r
         INNER JOIN problems p ON p.id = r.problem_id
         ORDER BY r.opened_at DESC
         LIMIT 6",
    )
    .fetch_all(pool)
    .await?;

    let modified_rows = sqlx::query(&format!(
        "{SELECT_PROBLEM} ORDER BY updated_at DESC, id DESC LIMIT 6"
    ))
    .fetch_all(pool)
    .await?;

    let platform_rows = sqlx::query(
        "SELECT DISTINCT platform FROM problems WHERE TRIM(platform) != '' ORDER BY platform COLLATE NOCASE ASC",
    )
    .fetch_all(pool)
    .await?;

    let tag_rows = sqlx::query("SELECT tags FROM problems WHERE tags != '[]'")
        .fetch_all(pool)
        .await?;
    let mut tag_set = HashSet::new();
    for r in tag_rows {
        let raw: String = r.get(0);
        if let Ok(list) = serde_json::from_str::<Vec<String>>(&raw) {
            for t in list {
                tag_set.insert(t);
            }
        }
    }
    let mut tags: Vec<String> = tag_set.into_iter().collect();
    tags.sort_by_key(|a| a.to_ascii_lowercase());

    Ok(DashboardStats {
        total,
        solved,
        in_progress,
        not_started,
        review,
        completion_percent,
        recent: recent_rows
            .iter()
            .map(row_to_problem)
            .collect::<CoreResult<Vec<_>>>()?,
        modified: modified_rows
            .iter()
            .map(row_to_problem)
            .collect::<CoreResult<Vec<_>>>()?,
        platforms: platform_rows.into_iter().map(|r| r.get(0)).collect(),
        tags,
    })
}

pub async fn clear_recent(pool: &SqlitePool) -> CoreResult<()> {
    sqlx::query("DELETE FROM recent_problems")
        .execute(pool)
        .await?;
    Ok(())
}

async fn ensure_identifier_available(
    pool: &SqlitePool,
    identifier: &str,
    except_id: Option<i64>,
) -> CoreResult<()> {
    if identifier.is_empty() {
        return Ok(());
    }
    let existing: Option<i64> = sqlx::query_scalar("SELECT id FROM problems WHERE identifier = ?1")
        .bind(identifier)
        .fetch_optional(pool)
        .await?;

    if existing.is_some() && existing != except_id {
        return Err(CoreError::Conflict(format!(
            "Another problem already uses identifier '{identifier}'"
        )));
    }
    Ok(())
}
