use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const PROBLEM_STATUSES: &[&str] = &["not_started", "in_progress", "solved", "review"];
pub const PROBLEM_DIFFICULTIES: &[&str] = &["easy", "medium", "hard", "custom"];
pub const FILE_LAYOUTS: &[&str] = &["auto", "subdirs", "flat"];
pub const TEMPLATE_MODES: &[&str] = &["starter", "empty"];
pub const THEMES: &[&str] = &["system", "light", "dark"];
pub const APP_LANGUAGES: &[&str] = &["system", "en", "vi"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: i64,
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub platform: String,
    pub url: String,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub status: String,
    pub languages: Vec<String>,
    pub workspace_path: String,
    pub folder_name: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_opened_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFile {
    pub name: String,
    pub relative_path: String,
    pub language: Option<String>,
    pub size_bytes: u64,
    pub is_readme: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetail {
    pub problem: Problem,
    pub files: Vec<WorkspaceFile>,
    pub workspace_exists: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct CreateProblemInput {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub platform: String,
    pub url: String,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub status: String,
    pub languages: Vec<String>,
    pub notes: String,
    pub folder_name: Option<String>,
    pub workspace_root: Option<String>,
    pub group_by_platform: Option<bool>,
    pub file_layout: Option<String>,
    pub template_mode: Option<String>,
    pub custom_filenames: HashMap<String, String>,
    pub generate_readme: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct UpdateProblemInput {
    pub identifier: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub platform: Option<String>,
    pub url: Option<String>,
    pub difficulty: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
    pub languages: Option<Vec<String>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct AddProblemFileInput {
    pub language_id: String,
    pub filename: Option<String>,
    pub template_id: Option<String>,
    pub subdir: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct GenerateReadmeInput {
    pub overwrite: bool,
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ProblemQuery {
    pub search: String,
    pub status: String,
    pub difficulty: String,
    pub language: String,
    pub platform: String,
    pub tag: String,
    pub sort: String,
    pub dir: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardStats {
    pub total: i64,
    pub solved: i64,
    pub in_progress: i64,
    pub not_started: i64,
    pub review: i64,
    pub completion_percent: f64,
    pub recent: Vec<Problem>,
    pub modified: Vec<Problem>,
    pub platforms: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub extension: String,
    pub default_filename: String,
    pub folder_name: String,
    pub is_builtin: bool,
    pub enabled: bool,
    pub sort_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLanguageInput {
    pub id: String,
    pub name: String,
    pub extension: String,
    pub default_filename: String,
    pub folder_name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct UpdateLanguageInput {
    pub name: Option<String>,
    pub extension: Option<String>,
    pub default_filename: Option<String>,
    pub folder_name: Option<String>,
    pub enabled: Option<bool>,
    pub sort_order: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub language_id: Option<String>,
    pub content: String,
    pub is_default: bool,
    pub is_builtin: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTemplateInput {
    pub name: String,
    pub kind: String,
    pub language_id: Option<String>,
    pub content: String,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct UpdateTemplateInput {
    pub name: Option<String>,
    pub content: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub workspace_path: String,
    pub theme: String,
    pub language: String,
    pub default_language: String,
    pub default_difficulty: String,
    pub default_status: String,
    pub default_template_mode: String,
    pub group_by_platform: bool,
    pub file_layout: String,
    pub generate_readme: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct SettingsPatch {
    pub workspace_path: Option<String>,
    pub theme: Option<String>,
    pub language: Option<String>,
    pub default_language: Option<String>,
    pub default_difficulty: Option<String>,
    pub default_status: Option<String>,
    pub default_template_mode: Option<String>,
    pub group_by_platform: Option<bool>,
    pub file_layout: Option<String>,
    pub generate_readme: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceStatus {
    pub path: String,
    pub exists: bool,
    pub writable: bool,
    pub valid: bool,
    pub problem_folders: usize,
    pub message: Option<String>,
}
