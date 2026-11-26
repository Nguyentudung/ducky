use crate::core::{
    error::{AppError, AppResult},
    workspace::SourceFile,
};
use serde::{Deserialize, Serialize};

const STATUSES: &[&str] = &["not_started", "in_progress", "solved", "review"];
const DIFFICULTIES: &[&str] = &["easy", "medium", "hard", "custom"];

#[derive(Debug, Clone, Serialize)]
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
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_opened_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProblemDetail {
    pub problem: Problem,
    pub source_files: Vec<SourceFile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProblemInput {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub platform: String,
    pub url: String,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub status: String,
    pub languages: Vec<String>,
    pub workspace_path: Option<String>,
    pub notes: String,
    pub template: String,
}

impl Default for ProblemInput {
    fn default() -> Self {
        Self {
            identifier: String::new(),
            title: String::new(),
            description: String::new(),
            platform: String::new(),
            url: String::new(),
            difficulty: "medium".into(),
            tags: vec![],
            status: "not_started".into(),
            languages: vec!["rust".into()],
            workspace_path: None,
            notes: String::new(),
            template: "starter".into(),
        }
    }
}

impl ProblemInput {
    pub fn normalized(mut self) -> AppResult<Self> {
        self.title = required(&self.title, "Problem title", 160)?;
        self.identifier = self.identifier.trim().to_string();
        if self.identifier.len() > 80
            || !self
                .identifier
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(AppError::Validation(
                "Identifier may contain only letters, numbers, hyphens, and underscores".into(),
            ));
        }
        self.description = optional(&self.description, "Description", 20_000)?;
        self.platform = optional(&self.platform, "Platform", 80)?;
        self.url = self.url.trim().to_string();
        if !self.url.is_empty()
            && !(self.url.starts_with("https://") || self.url.starts_with("http://"))
        {
            return Err(AppError::Validation(
                "Problem URL must start with http:// or https://".into(),
            ));
        }
        if self.url.len() > 2_000 {
            return Err(AppError::Validation("Problem URL is too long".into()));
        }
        self.difficulty = self.difficulty.trim().to_ascii_lowercase();
        if !DIFFICULTIES.contains(&self.difficulty.as_str()) {
            return Err(AppError::Validation("Invalid problem difficulty".into()));
        }
        self.status = self.status.trim().to_ascii_lowercase();
        if !STATUSES.contains(&self.status.as_str()) {
            return Err(AppError::Validation("Invalid problem status".into()));
        }
        self.template = self.template.trim().to_ascii_lowercase();
        if self.template.is_empty() {
            self.template = "starter".into();
        }
        if !["starter", "empty"].contains(&self.template.as_str()) {
            return Err(AppError::Validation("Invalid source-file template".into()));
        }
        self.tags = list(self.tags, "tag", 20, 40)?;
        self.languages = list(self.languages, "language", 5, 20)?;
        if self.languages.is_empty() {
            return Err(AppError::Validation(
                "Select at least one programming language".into(),
            ));
        }
        for language in &self.languages {
            if !crate::core::workspace::supported_languages().contains(&language.as_str()) {
                return Err(AppError::Validation(format!(
                    "Unsupported programming language: {language}"
                )));
            }
        }
        self.notes = optional(&self.notes, "Notes", 50_000)?;
        self.workspace_path = self
            .workspace_path
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        Ok(self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProblemQuery {
    pub search: String,
    pub status: String,
    pub difficulty: String,
    pub language: String,
    pub platform: String,
    pub tag: String,
    pub sort: String,
    pub sort_direction: String,
}

impl ProblemQuery {
    pub fn with_defaults(mut self) -> Self {
        if self.sort.is_empty() {
            self.sort = "modified".into();
        }
        if self.sort_direction.is_empty() {
            self.sort_direction = "desc".into();
        }
        self
    }
}

#[derive(Debug, Clone)]
pub struct Dashboard {
    pub total: i64,
    pub solved: i64,
    pub in_progress: i64,
    pub not_started: i64,
    pub review: i64,
    pub recent: Vec<Problem>,
    pub modified: Vec<Problem>,
}

fn required(v: &str, label: &str, limit: usize) -> AppResult<String> {
    let value = optional(v, label, limit)?;
    if value.is_empty() {
        Err(AppError::Validation(format!("{label} is required")))
    } else {
        Ok(value)
    }
}
fn optional(v: &str, label: &str, limit: usize) -> AppResult<String> {
    let value = v.trim().to_string();
    if value.chars().count() > limit {
        Err(AppError::Validation(format!("{label} is too long")))
    } else {
        Ok(value)
    }
}
fn list(
    values: Vec<String>,
    label: &str,
    maximum: usize,
    item_limit: usize,
) -> AppResult<Vec<String>> {
    let mut result = vec![];
    for value in values {
        let value = value.trim().to_string();
        if value.is_empty() {
            continue;
        }
        if value.chars().count() > item_limit {
            return Err(AppError::Validation(format!("A {label} is too long")));
        }
        if !result.contains(&value) {
            result.push(value);
        }
    }
    if result.len() > maximum {
        Err(AppError::Validation(format!("Too many {label}s selected")))
    } else {
        Ok(result)
    }
}
