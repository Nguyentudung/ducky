use crate::platform::open_folder;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ducky_core::{
    error::CoreError,
    languages::{create_language, delete_language, list_languages, update_language},
    models::{
        AddProblemFileInput, CreateLanguageInput, CreateProblemInput, CreateTemplateInput,
        GenerateReadmeInput, ProblemQuery, SettingsPatch, UpdateLanguageInput, UpdateProblemInput,
        UpdateTemplateInput,
    },
    problems::{
        add_problem_file, clear_recent, create_problem, dashboard_stats, delete_problem,
        generate_problem_readme, get_problem, get_problem_detail, list_problems, update_problem,
    },
    settings::{get_settings, patch_settings},
    templates::{create_template, delete_template, list_templates, update_template},
    workspace::{ensure_within_workspace, validate_workspace_path},
};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;

pub struct ApiError(pub CoreError);

impl From<CoreError> for ApiError {
    fn from(value: CoreError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            CoreError::Validation(_) | CoreError::Serialization(_) => StatusCode::BAD_REQUEST,
            CoreError::NotFound(_) => StatusCode::NOT_FOUND,
            CoreError::Conflict(_) => StatusCode::CONFLICT,
            CoreError::Database(_) | CoreError::Migration(_) | CoreError::Filesystem { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!(error = %self.0, "Internal backend error");
        }
        (
            status,
            Json(json!({
                "error": {
                    "code": self.0.code(),
                    "message": self.0.safe_message()
                }
            })),
        )
            .into_response()
    }
}

type ApiResult<T> = Result<T, ApiError>;

pub async fn health(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let db_ok: i64 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .map_err(CoreError::Database)?;

    Ok(Json(json!({
        "status": "ok",
        "version": state.version,
        "host": state.host,
        "port": state.port,
        "database": db_ok == 1
    })))
}

pub async fn get_dashboard(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let stats = dashboard_stats(&state.pool).await?;
    let settings = get_settings(&state.pool, &state.default_workspace).await?;
    let workspace = validate_workspace_path(std::path::Path::new(&settings.workspace_path));
    Ok(Json(json!({
        "stats": stats,
        "workspace": workspace
    })))
}

pub async fn problems_list(
    State(state): State<AppState>,
    Query(query): Query<ProblemQuery>,
) -> ApiResult<impl IntoResponse> {
    let problems = list_problems(&state.pool, query).await?;
    Ok(Json(json!({ "problems": problems })))
}

pub async fn problems_create(
    State(state): State<AppState>,
    Json(input): Json<CreateProblemInput>,
) -> ApiResult<impl IntoResponse> {
    let detail = create_problem(&state.pool, &state.default_workspace, input).await?;
    Ok((StatusCode::CREATED, Json(detail)))
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct DetailQuery {
    pub open: Option<u8>,
}

pub async fn problems_detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<DetailQuery>,
) -> ApiResult<impl IntoResponse> {
    let record_open = query.open == Some(1);
    let detail = get_problem_detail(&state.pool, id, record_open).await?;
    Ok(Json(detail))
}

pub async fn problems_update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateProblemInput>,
) -> ApiResult<impl IntoResponse> {
    let problem = update_problem(&state.pool, id, input).await?;
    Ok(Json(json!({ "problem": problem })))
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct DeleteProblemQuery {
    pub delete_files: Option<bool>,
}

pub async fn problems_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<DeleteProblemQuery>,
) -> ApiResult<impl IntoResponse> {
    let delete_files = query.delete_files.unwrap_or(false);
    delete_problem(&state.pool, &state.default_workspace, id, delete_files).await?;
    Ok(Json(
        json!({ "deleted": true, "files_deleted": delete_files }),
    ))
}

pub async fn problems_add_file(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<AddProblemFileInput>,
) -> ApiResult<impl IntoResponse> {
    let file = add_problem_file(&state.pool, &state.default_workspace, id, input).await?;
    Ok((StatusCode::CREATED, Json(json!({ "file": file }))))
}

pub async fn problems_generate_readme(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<GenerateReadmeInput>,
) -> ApiResult<impl IntoResponse> {
    generate_problem_readme(&state.pool, id, input).await?;
    Ok(Json(json!({ "generated": true })))
}

pub async fn problems_open_folder(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    let problem = get_problem(&state.pool, id).await?;
    if problem.workspace_path.is_empty() {
        return Err(
            CoreError::Validation("This problem does not have a workspace folder".into()).into(),
        );
    }
    let settings = get_settings(&state.pool, &state.default_workspace).await?;
    let ws_root = PathBuf::from(&settings.workspace_path);
    let target = PathBuf::from(&problem.workspace_path);

    // If folder is inside configured workspace root, verify no symlink escape.
    if ws_root.exists()
        && let Ok(canon) = ensure_within_workspace(&ws_root, &target)
    {
        open_folder(&canon).map_err(|e| CoreError::Validation(e.to_string()))?;
        return Ok(Json(json!({ "opened": true })));
    }

    if !target.is_dir() {
        return Err(CoreError::NotFound(
            "Problem workspace directory no longer exists on disk".into(),
        )
        .into());
    }
    open_folder(&target).map_err(|e| CoreError::Validation(e.to_string()))?;
    Ok(Json(json!({ "opened": true })))
}

pub async fn recent_clear(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    clear_recent(&state.pool).await?;
    Ok(Json(json!({ "cleared": true })))
}

pub async fn languages_list(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let languages = list_languages(&state.pool).await?;
    Ok(Json(json!({ "languages": languages })))
}

pub async fn languages_create(
    State(state): State<AppState>,
    Json(input): Json<CreateLanguageInput>,
) -> ApiResult<impl IntoResponse> {
    let language = create_language(&state.pool, input).await?;
    Ok((StatusCode::CREATED, Json(json!({ "language": language }))))
}

pub async fn languages_update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<UpdateLanguageInput>,
) -> ApiResult<impl IntoResponse> {
    let language = update_language(&state.pool, &id, input).await?;
    Ok(Json(json!({ "language": language })))
}

pub async fn languages_delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    delete_language(&state.pool, &id).await?;
    Ok(Json(json!({ "deleted": true })))
}

pub async fn templates_list(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let templates = list_templates(&state.pool).await?;
    Ok(Json(json!({ "templates": templates })))
}

pub async fn templates_create(
    State(state): State<AppState>,
    Json(input): Json<CreateTemplateInput>,
) -> ApiResult<impl IntoResponse> {
    let template = create_template(&state.pool, input).await?;
    Ok((StatusCode::CREATED, Json(json!({ "template": template }))))
}

pub async fn templates_update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<UpdateTemplateInput>,
) -> ApiResult<impl IntoResponse> {
    let template = update_template(&state.pool, &id, input).await?;
    Ok(Json(json!({ "template": template })))
}

pub async fn templates_delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    delete_template(&state.pool, &id).await?;
    Ok(Json(json!({ "deleted": true })))
}

pub async fn settings_get(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let settings = get_settings(&state.pool, &state.default_workspace).await?;
    Ok(Json(json!({ "settings": settings })))
}

pub async fn settings_patch(
    State(state): State<AppState>,
    Json(patch): Json<SettingsPatch>,
) -> ApiResult<impl IntoResponse> {
    let settings = patch_settings(&state.pool, &state.default_workspace, patch).await?;
    Ok(Json(json!({ "settings": settings })))
}

pub async fn workspace_get(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let settings = get_settings(&state.pool, &state.default_workspace).await?;
    let status = validate_workspace_path(std::path::Path::new(&settings.workspace_path));
    Ok(Json(json!({ "workspace": status })))
}

#[derive(Deserialize)]
pub struct ValidateWorkspaceInput {
    pub path: String,
}

pub async fn workspace_validate(
    Json(input): Json<ValidateWorkspaceInput>,
) -> ApiResult<impl IntoResponse> {
    let status = validate_workspace_path(std::path::Path::new(input.path.trim()));
    Ok(Json(json!({ "workspace": status })))
}

pub async fn workspace_open(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let settings = get_settings(&state.pool, &state.default_workspace).await?;
    let dir = PathBuf::from(&settings.workspace_path);
    if !dir.is_dir() {
        return Err(CoreError::Validation(
            "Configured workspace directory does not exist yet".into(),
        )
        .into());
    }
    open_folder(&dir).map_err(|e| CoreError::Validation(e.to_string()))?;
    Ok(Json(json!({ "opened": true })))
}
