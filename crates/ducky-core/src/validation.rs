use crate::error::{CoreError, CoreResult};
use crate::models::{
    APP_LANGUAGES, FILE_LAYOUTS, PROBLEM_DIFFICULTIES, PROBLEM_STATUSES, TEMPLATE_MODES, THEMES,
};

const RESERVED_WINDOWS_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

pub fn required_text(raw: &str, label: &str, max_chars: usize) -> CoreResult<String> {
    let trimmed = optional_text(raw, label, max_chars)?;
    if trimmed.is_empty() {
        Err(CoreError::Validation(format!("{label} is required")))
    } else {
        Ok(trimmed)
    }
}

pub fn optional_text(raw: &str, label: &str, max_chars: usize) -> CoreResult<String> {
    let trimmed = raw.trim().to_string();
    if trimmed.chars().count() > max_chars {
        Err(CoreError::Validation(format!(
            "{label} exceeds maximum length of {max_chars} characters"
        )))
    } else {
        Ok(trimmed)
    }
}

pub fn normalize_identifier(raw: &str) -> CoreResult<String> {
    let trimmed = raw.trim().to_string();
    if trimmed.len() > 80 {
        return Err(CoreError::Validation(
            "Identifier exceeds maximum length of 80 characters".into(),
        ));
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(CoreError::Validation(
            "Identifier may contain only ASCII letters, numbers, hyphens, and underscores".into(),
        ));
    }
    Ok(trimmed)
}

pub fn normalize_url(raw: &str) -> CoreResult<String> {
    let trimmed = raw.trim().to_string();
    if trimmed.is_empty() {
        return Ok(trimmed);
    }
    if trimmed.len() > 2_000 {
        return Err(CoreError::Validation("Problem URL is too long".into()));
    }
    if !(trimmed.starts_with("https://") || trimmed.starts_with("http://")) {
        return Err(CoreError::Validation(
            "Problem URL must start with http:// or https://".into(),
        ));
    }
    Ok(trimmed)
}

pub fn normalize_enum(raw: &str, allowed: &[&str], label: &str) -> CoreResult<String> {
    let normalized = raw.trim().to_ascii_lowercase();
    if allowed.contains(&normalized.as_str()) {
        Ok(normalized)
    } else {
        Err(CoreError::Validation(format!(
            "Invalid {label}: expected one of {}",
            allowed.join(", ")
        )))
    }
}

pub fn normalize_status(raw: &str) -> CoreResult<String> {
    normalize_enum(raw, PROBLEM_STATUSES, "problem status")
}

pub fn normalize_difficulty(raw: &str) -> CoreResult<String> {
    normalize_enum(raw, PROBLEM_DIFFICULTIES, "problem difficulty")
}

pub fn normalize_file_layout(raw: &str) -> CoreResult<String> {
    normalize_enum(raw, FILE_LAYOUTS, "file layout")
}

pub fn normalize_template_mode(raw: &str) -> CoreResult<String> {
    normalize_enum(raw, TEMPLATE_MODES, "template mode")
}

pub fn normalize_theme(raw: &str) -> CoreResult<String> {
    normalize_enum(raw, THEMES, "theme")
}

pub fn normalize_app_language(raw: &str) -> CoreResult<String> {
    normalize_enum(raw, APP_LANGUAGES, "application language")
}

pub fn normalize_list(
    values: Vec<String>,
    label: &str,
    max_items: usize,
    max_item_chars: usize,
) -> CoreResult<Vec<String>> {
    let mut out = Vec::new();
    for item in values {
        let trimmed = item.trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.chars().count() > max_item_chars {
            return Err(CoreError::Validation(format!(
                "A {label} exceeds {max_item_chars} characters"
            )));
        }
        if !out.contains(&trimmed) {
            out.push(trimmed);
        }
    }
    if out.len() > max_items {
        return Err(CoreError::Validation(format!(
            "Too many {label}s (maximum {max_items})"
        )));
    }
    Ok(out)
}

pub fn normalize_slug_id(raw: &str, label: &str) -> CoreResult<String> {
    let normalized = raw.trim().to_ascii_lowercase();
    if normalized.is_empty() || normalized.len() > 40 {
        return Err(CoreError::Validation(format!(
            "{label} must be between 1 and 40 characters"
        )));
    }
    if !normalized
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(CoreError::Validation(format!(
            "{label} may only contain lowercase letters, numbers, hyphens, and underscores"
        )));
    }
    Ok(normalized)
}

/// Validates a user-provided single filename or folder name (never a path with separators).
pub fn validate_single_component(raw: &str, label: &str) -> CoreResult<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(CoreError::Validation(format!("{label} cannot be empty")));
    }
    if trimmed.chars().count() > 120 {
        return Err(CoreError::Validation(format!(
            "{label} exceeds 120 characters"
        )));
    }
    if trimmed == "." || trimmed == ".." || trimmed.ends_with('.') {
        return Err(CoreError::Validation(format!(
            "{label} contains an invalid trailing dot or relative segment"
        )));
    }
    if trimmed.chars().any(|c| {
        matches!(
            c,
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'
        ) || c.is_control()
    }) {
        return Err(CoreError::Validation(format!(
            "{label} contains disallowed path or control characters"
        )));
    }
    let stem = trimmed
        .split('.')
        .next()
        .unwrap_or(trimmed)
        .to_ascii_uppercase();
    if RESERVED_WINDOWS_NAMES.contains(&stem.as_str()) {
        return Err(CoreError::Validation(format!(
            "{label} cannot use a reserved OS device name"
        )));
    }
    Ok(trimmed.to_string())
}

/// Converts an arbitrary title or platform name into a safe single directory component.
pub fn derive_safe_folder_name(raw: &str, label: &str) -> CoreResult<String> {
    let sanitized: String = raw
        .trim()
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' | '\0' => '-',
            c if c.is_control() => '-',
            c => c,
        })
        .collect();
    let trimmed = sanitized
        .trim_matches(|c: char| c == '.' || c.is_whitespace() || c == '-')
        .chars()
        .take(96)
        .collect::<String>();
    if trimmed.is_empty() || trimmed == "." || trimmed == ".." {
        return Err(CoreError::Validation(format!(
            "{label} cannot produce a safe folder name"
        )));
    }
    let stem = trimmed.to_ascii_uppercase();
    if RESERVED_WINDOWS_NAMES.contains(&stem.as_str()) {
        return Ok(format!("{trimmed}-dir"));
    }
    Ok(trimmed)
}
