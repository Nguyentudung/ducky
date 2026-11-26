//! Safe creation and validation of on-disk problem workspaces.

use crate::core::error::{AppError, AppResult};
use serde::Serialize;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const PROBLEMS_DIRECTORY: &str = "Problems";

#[derive(Debug, Clone, Serialize)]
pub struct SourceFile {
    pub language: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct ProblemManifest<'a> {
    pub id: Option<i64>,
    pub identifier: &'a str,
    pub title: &'a str,
    pub platform: &'a str,
    pub url: &'a str,
    pub difficulty: &'a str,
    pub tags: &'a [String],
    pub status: &'a str,
    pub languages: &'a [String],
    pub description: &'a str,
    pub notes: &'a str,
}

/// Validate a user-selected directory, create its Ducky child structure, and canonicalize it.
pub fn ensure_workspace_root(path: &Path) -> AppResult<PathBuf> {
    if !path.is_absolute() {
        return Err(AppError::Validation(
            "Workspace path must be an absolute path".into(),
        ));
    }
    if path.exists() && !path.is_dir() {
        return Err(AppError::Validation(
            "Workspace path points to a file, not a directory".into(),
        ));
    }
    fs::create_dir_all(path)
        .map_err(|e| AppError::filesystem("Could not create workspace directory", e))?;
    let canonical = path
        .canonicalize()
        .map_err(|e| AppError::filesystem("Could not validate workspace directory", e))?;
    fs::create_dir_all(canonical.join(PROBLEMS_DIRECTORY))
        .map_err(|e| AppError::filesystem("Could not create Problems directory", e))?;
    Ok(canonical)
}

/// Creates in a temporary sibling directory first. Existing user files are never overwritten.
pub fn create_problem_workspace(
    root: &Path,
    title: &str,
    languages: &[String],
    template: &str,
    manifest: &ProblemManifest<'_>,
) -> AppResult<PathBuf> {
    let root = ensure_workspace_root(root)?;
    let folder = safe_folder_name(title)?;
    let problems = root.join(PROBLEMS_DIRECTORY);
    let destination = problems.join(&folder);
    if destination.exists() {
        return Err(AppError::Validation(format!(
            "A workspace named '{folder}' already exists"
        )));
    }
    let temporary = problems.join(temp_name());
    fs::create_dir(&temporary)
        .map_err(|e| AppError::filesystem("Could not prepare problem workspace", e))?;
    let result = (|| {
        write_files(&temporary, languages, template, manifest)?;
        fs::rename(&temporary, &destination).map_err(|e| {
            AppError::filesystem(
                "Could not finalize problem workspace; no existing files were overwritten",
                e,
            )
        })?;
        Ok(destination)
    })();
    if result.is_err() && temporary.exists() {
        let _ = fs::remove_dir_all(&temporary);
    }
    result
}

/// The manifest is app-owned metadata; source and notes files remain fully user-owned.
pub fn write_problem_manifest(path: &Path, manifest: &ProblemManifest<'_>) -> AppResult<()> {
    if !path.is_dir() {
        return Err(AppError::Validation(
            "The problem workspace no longer exists on disk".into(),
        ));
    }
    atomic(
        &path.join("problem.json"),
        &serde_json::to_string_pretty(manifest)?,
    )
}
pub fn source_files(workspace: &Path, languages: &[String]) -> Vec<SourceFile> {
    languages
        .iter()
        .filter_map(|language| {
            let relative = Path::new("solutions")
                .join(language)
                .join(source_filename(language)?);
            workspace.join(&relative).is_file().then(|| SourceFile {
                language: language.clone(),
                path: relative.to_string_lossy().into_owned(),
            })
        })
        .collect()
}
pub fn supported_languages() -> &'static [&'static str] {
    &["c", "cpp", "rust", "python", "java"]
}

fn write_files(
    dir: &Path,
    languages: &[String],
    template: &str,
    manifest: &ProblemManifest<'_>,
) -> AppResult<()> {
    new(
        &dir.join("problem.json"),
        &serde_json::to_string_pretty(manifest)?,
    )?;
    new(
        &dir.join("README.md"),
        &format!(
            "# {}\n\n{}\n\n- Platform: {}\n- Difficulty: {}\n",
            manifest.title, manifest.description, manifest.platform, manifest.difficulty
        ),
    )?;
    new(&dir.join("notes.md"), "# Notes\n\n")?;
    for language in languages {
        let filename = source_filename(language).ok_or_else(|| {
            AppError::Validation(format!("Unsupported programming language: {language}"))
        })?;
        let source = dir.join("solutions").join(language);
        fs::create_dir_all(&source)
            .map_err(|e| AppError::filesystem("Could not create solution directory", e))?;
        new(
            &source.join(filename),
            &starter_template(language, template)?,
        )?;
    }
    Ok(())
}
fn new(path: &Path, contents: &str) -> AppResult<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| AppError::filesystem(format!("Could not create {}", path.display()), e))?;
    file.write_all(contents.as_bytes())
        .map_err(|e| AppError::filesystem(format!("Could not write {}", path.display()), e))?;
    file.sync_all()
        .map_err(|e| AppError::filesystem(format!("Could not save {}", path.display()), e))?;
    Ok(())
}
fn atomic(path: &Path, contents: &str) -> AppResult<()> {
    let temporary = path.with_extension("json.tmp");
    let mut file = File::create(&temporary)
        .map_err(|e| AppError::filesystem("Could not prepare problem manifest", e))?;
    file.write_all(contents.as_bytes())
        .map_err(|e| AppError::filesystem("Could not write problem manifest", e))?;
    file.sync_all()
        .map_err(|e| AppError::filesystem("Could not save problem manifest", e))?;
    fs::rename(temporary, path)
        .map_err(|e| AppError::filesystem("Could not replace problem manifest", e))?;
    Ok(())
}
fn safe_folder_name(title: &str) -> AppResult<String> {
    let name = title
        .trim()
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' | '\0' => '-',
            c if c.is_control() => '-',
            c => c,
        })
        .collect::<String>();
    let name = name.trim_matches(|c: char| c == '.' || c.is_whitespace());
    if name.is_empty() || name == "." || name == ".." {
        Err(AppError::Validation(
            "Problem title cannot produce a safe workspace folder name".into(),
        ))
    } else {
        Ok(name.chars().take(96).collect())
    }
}
fn temp_name() -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    format!(".ducky-create-{}-{nonce}", std::process::id())
}
fn source_filename(language: &str) -> Option<&'static str> {
    match language {
        "c" => Some("main.c"),
        "cpp" => Some("main.cpp"),
        "rust" => Some("main.rs"),
        "python" => Some("main.py"),
        "java" => Some("Main.java"),
        _ => None,
    }
}
fn starter_template(language: &str, template: &str) -> AppResult<String> {
    if template == "empty" {
        return Ok(String::new());
    }
    if template != "starter" {
        return Err(AppError::Validation("Unknown source-file template".into()));
    }
    Ok(match language{"c"=>"#include <stdio.h>\n\nint main(void) {\n    return 0;\n}\n","cpp"=>"#include <bits/stdc++.h>\nusing namespace std;\n\nint main() {\n    ios::sync_with_stdio(false);\n    cin.tie(nullptr);\n    return 0;\n}\n","rust"=>"fn main() {\n    // Solve the problem here.\n}\n","python"=>"def solve() -> None:\n    pass\n\n\nif __name__ == \"__main__\":\n    solve()\n","java"=>"import java.io.*;\nimport java.util.*;\n\npublic class Main {\n    public static void main(String[] args) throws Exception {\n        // Solve the problem here.\n    }\n}\n",_=>return Err(AppError::Validation(format!("Unsupported programming language: {language}"))) }.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn root() -> PathBuf {
        std::env::temp_dir().join(format!("ducky-test-{}", temp_name()))
    }

    #[test]
    fn creates_selected_language_files_without_overwriting() {
        let root = root();
        let languages = vec![
            "c".into(),
            "cpp".into(),
            "rust".into(),
            "python".into(),
            "java".into(),
        ];
        let tags = vec![];
        let manifest = ProblemManifest {
            id: None,
            identifier: "two-sum",
            title: "Two Sum",
            platform: "",
            url: "",
            difficulty: "easy",
            tags: &tags,
            status: "not_started",
            languages: &languages,
            description: "",
            notes: "",
        };
        let workspace =
            create_problem_workspace(&root, "Two Sum", &languages, "starter", &manifest).unwrap();
        for file in [
            "solutions/c/main.c",
            "solutions/cpp/main.cpp",
            "solutions/rust/main.rs",
            "solutions/python/main.py",
            "solutions/java/Main.java",
        ] {
            assert!(workspace.join(file).is_file(), "missing {file}");
        }
        assert!(
            create_problem_workspace(&root, "Two Sum", &languages, "starter", &manifest).is_err()
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_relative_workspace_paths() {
        assert!(ensure_workspace_root(Path::new("relative")).is_err());
    }
}
