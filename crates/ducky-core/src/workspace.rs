use crate::error::{CoreError, CoreResult};
use crate::models::{Language, WorkspaceFile, WorkspaceStatus};
use crate::validation::{derive_safe_folder_name, validate_single_component};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn ensure_workspace_root(path: &Path) -> CoreResult<PathBuf> {
    if !path.is_absolute() {
        return Err(CoreError::Validation(
            "Workspace path must be an absolute path".into(),
        ));
    }
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(CoreError::Validation(
            "Workspace path cannot contain '..' segments".into(),
        ));
    }
    if path.exists() && !path.is_dir() {
        return Err(CoreError::Validation(
            "Workspace path points to a file, not a directory".into(),
        ));
    }
    fs::create_dir_all(path)
        .map_err(|e| CoreError::filesystem("Could not create workspace directory", e))?;
    path.canonicalize()
        .map_err(|e| CoreError::filesystem("Could not resolve workspace directory", e))
}

/// Validates a candidate workspace path without creating directories unnecessarily.
pub fn validate_workspace_path(path: &Path) -> WorkspaceStatus {
    let raw = path.to_string_lossy().into_owned();
    if !path.is_absolute() {
        return WorkspaceStatus {
            path: raw,
            exists: false,
            writable: false,
            valid: false,
            problem_folders: 0,
            message: Some("Workspace path must be an absolute path".into()),
        };
    }
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return WorkspaceStatus {
            path: raw,
            exists: false,
            writable: false,
            valid: false,
            problem_folders: 0,
            message: Some("Workspace path cannot contain '..' segments".into()),
        };
    }
    if path.exists() {
        if !path.is_dir() {
            return WorkspaceStatus {
                path: raw,
                exists: true,
                writable: false,
                valid: false,
                problem_folders: 0,
                message: Some("Target path exists and is a file, not a directory".into()),
            };
        }
        let problem_folders = count_visible_subdirs(path);
        let writable = check_dir_writable(path);
        return WorkspaceStatus {
            path: path
                .canonicalize()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or(raw),
            exists: true,
            writable,
            valid: writable,
            problem_folders,
            message: if writable {
                None
            } else {
                Some("Directory exists but is not writable".into())
            },
        };
    }

    // Check nearest existing ancestor for writability
    let mut ancestor = path.parent();
    while let Some(a) = ancestor {
        if a.exists() {
            let writable = a.is_dir() && check_dir_writable(a);
            return WorkspaceStatus {
                path: raw,
                exists: false,
                writable,
                valid: writable,
                problem_folders: 0,
                message: if writable {
                    Some("Directory does not exist yet and will be created".into())
                } else {
                    Some("Parent directory is not writable".into())
                },
            };
        }
        ancestor = a.parent();
    }

    WorkspaceStatus {
        path: raw,
        exists: false,
        writable: false,
        valid: false,
        problem_folders: 0,
        message: Some("Parent directory does not exist".into()),
    }
}

fn check_dir_writable(dir: &Path) -> bool {
    let probe = dir.join(format!(".ducky-probe-{}", temp_nonce()));
    match OpenOptions::new().write(true).create_new(true).open(&probe) {
        Ok(file) => {
            drop(file);
            let _ = fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

fn count_visible_subdirs(dir: &Path) -> usize {
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .filter(|e| {
            let name = e.file_name();
            let s = name.to_string_lossy();
            !s.starts_with('.') && e.file_type().map(|t| t.is_dir()).unwrap_or(false)
        })
        .count()
}

pub fn ensure_within_workspace(root: &Path, candidate: &Path) -> CoreResult<PathBuf> {
    let canon_root = root
        .canonicalize()
        .map_err(|e| CoreError::filesystem("Could not resolve workspace root", e))?;
    let canon_candidate = candidate
        .canonicalize()
        .map_err(|e| CoreError::filesystem("Could not resolve target path", e))?;

    if !canon_candidate.starts_with(&canon_root) {
        return Err(CoreError::Validation(
            "Target path escapes the configured workspace root".into(),
        ));
    }
    Ok(canon_candidate)
}

pub struct PlannedSourceFile {
    pub language: Language,
    pub filename: String,
    pub content: String,
}

pub struct CreateWorkspaceParams<'a> {
    pub workspace_root: &'a Path,
    pub title: &'a str,
    pub platform: &'a str,
    pub custom_folder_name: Option<&'a str>,
    pub group_by_platform: bool,
    pub file_layout: &'a str,
    pub files: &'a [PlannedSourceFile],
    pub readme_content: Option<&'a str>,
}

/// Atomically creates a new problem workspace folder and its starter files without overwriting.
pub fn create_problem_workspace(
    params: CreateWorkspaceParams<'_>,
) -> CoreResult<(PathBuf, String)> {
    let root = ensure_workspace_root(params.workspace_root)?;

    let folder_name = match params
        .custom_folder_name
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(custom) => validate_single_component(custom, "Folder name")?,
        None => derive_safe_folder_name(params.title, "Problem title")?,
    };

    let parent_dir = if params.group_by_platform && !params.platform.trim().is_empty() {
        let platform_dir_name = derive_safe_folder_name(params.platform, "Platform")?;
        let dir = root.join(platform_dir_name);
        if dir.exists() {
            let meta = fs::symlink_metadata(&dir)
                .map_err(|e| CoreError::filesystem("Could not inspect platform folder", e))?;
            if meta.file_type().is_symlink() || !meta.is_dir() {
                return Err(CoreError::Validation(
                    "Platform folder path is not a safe directory".into(),
                ));
            }
        } else {
            fs::create_dir_all(&dir)
                .map_err(|e| CoreError::filesystem("Could not create platform folder", e))?;
        }
        ensure_within_workspace(&root, &dir)?
    } else {
        root.clone()
    };

    let destination = parent_dir.join(&folder_name);
    if destination.exists() {
        return Err(CoreError::Conflict(format!(
            "A workspace folder named '{folder_name}' already exists"
        )));
    }

    let use_subdirs = match params.file_layout {
        "subdirs" => true,
        "flat" => false,
        _ => params.files.len() > 1,
    };

    // Check filename collisions if flat layout with multiple languages
    if !use_subdirs {
        let mut seen = std::collections::HashSet::new();
        for f in params.files {
            if !seen.insert(f.filename.clone()) {
                return Err(CoreError::Validation(format!(
                    "Duplicate filename '{}' in flat layout; switch layout to language subdirectories or customize filenames",
                    f.filename
                )));
            }
        }
    }

    let staging = parent_dir.join(format!(".ducky-staging-{}", temp_nonce()));
    fs::create_dir(&staging)
        .map_err(|e| CoreError::filesystem("Could not prepare problem folder", e))?;

    let outcome = (|| -> CoreResult<()> {
        if let Some(readme) = params.readme_content {
            write_new_file(&staging.join("README.md"), readme)?;
        }

        for file in params.files {
            let target_dir = if use_subdirs {
                let sub = staging.join(&file.language.folder_name);
                fs::create_dir_all(&sub).map_err(|e| {
                    CoreError::filesystem("Could not create language subdirectory", e)
                })?;
                sub
            } else {
                staging.clone()
            };
            write_new_file(&target_dir.join(&file.filename), &file.content)?;
        }

        fs::rename(&staging, &destination).map_err(|e| {
            CoreError::filesystem(
                "Could not finalize problem workspace without overwriting",
                e,
            )
        })?;
        Ok(())
    })();

    if outcome.is_err() && staging.exists() {
        let _ = fs::remove_dir_all(&staging);
    }
    outcome?;

    let canonical_dest = ensure_within_workspace(&root, &destination)?;
    Ok((canonical_dest, folder_name))
}

pub fn add_language_file_to_workspace(
    problem_dir: &Path,
    language: &Language,
    custom_filename: Option<&str>,
    content: &str,
    use_subdir: bool,
) -> CoreResult<WorkspaceFile> {
    if !problem_dir.is_dir() {
        return Err(CoreError::Validation(
            "Problem workspace directory does not exist on disk".into(),
        ));
    }
    let meta = fs::symlink_metadata(problem_dir)
        .map_err(|e| CoreError::filesystem("Could not inspect problem workspace", e))?;
    if meta.file_type().is_symlink() {
        return Err(CoreError::Validation(
            "Problem workspace cannot be a symlink".into(),
        ));
    }

    let filename = match custom_filename.map(str::trim).filter(|s| !s.is_empty()) {
        Some(name) => validate_single_component(name, "Filename")?,
        None => language.default_filename.clone(),
    };

    let target_dir = if use_subdir {
        let dir = problem_dir.join(&language.folder_name);
        if dir.exists() {
            let sub_meta = fs::symlink_metadata(&dir)
                .map_err(|e| CoreError::filesystem("Could not inspect language directory", e))?;
            if sub_meta.file_type().is_symlink() || !sub_meta.is_dir() {
                return Err(CoreError::Validation(
                    "Language subdirectory is not a valid directory".into(),
                ));
            }
        } else {
            fs::create_dir_all(&dir)
                .map_err(|e| CoreError::filesystem("Could not create language directory", e))?;
        }
        dir
    } else {
        problem_dir.to_path_buf()
    };

    let target_file = target_dir.join(&filename);
    if target_file.exists() {
        return Err(CoreError::Conflict(format!(
            "File '{}' already exists and will not be overwritten",
            target_file
                .strip_prefix(problem_dir)
                .unwrap_or(&target_file)
                .to_string_lossy()
        )));
    }

    write_new_file(&target_file, content)?;

    let relative = target_file
        .strip_prefix(problem_dir)
        .unwrap_or(&target_file)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(WorkspaceFile {
        name: filename,
        relative_path: relative,
        language: Some(language.id.clone()),
        size_bytes: content.len() as u64,
        is_readme: false,
    })
}

pub fn write_readme_file(problem_dir: &Path, content: &str, overwrite: bool) -> CoreResult<()> {
    if !problem_dir.is_dir() {
        return Err(CoreError::Validation(
            "Problem workspace directory does not exist on disk".into(),
        ));
    }
    let readme_path = problem_dir.join("README.md");
    if readme_path.exists() {
        if !overwrite {
            return Err(CoreError::Conflict(
                "README.md already exists; confirm overwrite to replace it".into(),
            ));
        }
        atomic_write(&readme_path, content)?;
    } else {
        write_new_file(&readme_path, content)?;
    }
    Ok(())
}

pub fn list_problem_files(problem_dir: &Path, languages: &[Language]) -> Vec<WorkspaceFile> {
    if !problem_dir.is_dir() {
        return Vec::new();
    }
    let Ok(canon_dir) = problem_dir.canonicalize() else {
        return Vec::new();
    };

    let mut ext_to_lang: HashMap<&str, &str> = HashMap::new();
    for lang in languages {
        ext_to_lang.insert(lang.extension.as_str(), lang.id.as_str());
    }

    let mut files = Vec::new();
    collect_files_shallow(&canon_dir, &canon_dir, 0, &ext_to_lang, &mut files);
    files.sort_by(|a, b| {
        b.is_readme
            .cmp(&a.is_readme)
            .then_with(|| a.relative_path.cmp(&b.relative_path))
    });
    files
}

fn collect_files_shallow(
    root: &Path,
    dir: &Path,
    depth: usize,
    ext_to_lang: &HashMap<&str, &str>,
    out: &mut Vec<WorkspaceFile>,
) {
    if depth > 2 {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let path = entry.path();
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            collect_files_shallow(root, &path, depth + 1, ext_to_lang, out);
        } else if meta.is_file() {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_ascii_lowercase());
            let is_readme = name.eq_ignore_ascii_case("README.md");
            let language = if is_readme {
                None
            } else {
                ext.as_deref()
                    .and_then(|e| ext_to_lang.get(e).copied())
                    .map(str::to_string)
            };
            out.push(WorkspaceFile {
                name,
                relative_path: rel,
                language,
                size_bytes: meta.len(),
                is_readme,
            });
        }
    }
}

pub fn delete_problem_directory(workspace_root: &Path, problem_dir: &Path) -> CoreResult<()> {
    if !problem_dir.exists() {
        return Ok(());
    }
    let canon_root = workspace_root
        .canonicalize()
        .map_err(|e| CoreError::filesystem("Could not resolve workspace root", e))?;
    let canon_target = ensure_within_workspace(&canon_root, problem_dir)?;
    if canon_target == canon_root {
        return Err(CoreError::Validation(
            "Refusing to delete the workspace root directory itself".into(),
        ));
    }
    fs::remove_dir_all(&canon_target)
        .map_err(|e| CoreError::filesystem("Could not delete problem directory", e))
}

fn write_new_file(path: &Path, contents: &str) -> CoreResult<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| CoreError::filesystem(format!("Could not create {}", path.display()), e))?;
    file.write_all(contents.as_bytes())
        .map_err(|e| CoreError::filesystem(format!("Could not write {}", path.display()), e))?;
    file.sync_all()
        .map_err(|e| CoreError::filesystem(format!("Could not flush {}", path.display()), e))?;
    Ok(())
}

fn atomic_write(path: &Path, contents: &str) -> CoreResult<()> {
    let tmp = path.with_extension(format!("tmp-{}", temp_nonce()));
    let mut file = File::create(&tmp)
        .map_err(|e| CoreError::filesystem("Could not prepare temporary file", e))?;
    file.write_all(contents.as_bytes())
        .map_err(|e| CoreError::filesystem("Could not write temporary file", e))?;
    file.sync_all()
        .map_err(|e| CoreError::filesystem("Could not flush temporary file", e))?;
    fs::rename(&tmp, path)
        .map_err(|e| CoreError::filesystem("Could not replace target file", e))?;
    Ok(())
}

fn temp_nonce() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{}-{nanos}", std::process::id())
}
