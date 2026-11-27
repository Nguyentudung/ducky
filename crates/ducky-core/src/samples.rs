use crate::error::CoreResult;
use sqlx::{Row, SqlitePool};
use std::path::{Path, PathBuf};

pub struct BuiltinSample {
    pub template_id: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub language_id: Option<&'static str>,
    pub rel_path: &'static str,
    pub fallback_content: &'static str,
}

pub const BUILTIN_SAMPLES: &[BuiltinSample] = &[
    BuiltinSample {
        template_id: "starter-c",
        name: "C Starter",
        kind: "source",
        language_id: Some("c"),
        rel_path: "c/main.c",
        fallback_content: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/c/main.c")),
    },
    BuiltinSample {
        template_id: "starter-cpp",
        name: "C++ Competitive Starter",
        kind: "source",
        language_id: Some("cpp"),
        rel_path: "cpp/main.cpp",
        fallback_content: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../samples/cpp/main.cpp"
        )),
    },
    BuiltinSample {
        template_id: "starter-python",
        name: "Python Starter",
        kind: "source",
        language_id: Some("python"),
        rel_path: "python/main.py",
        fallback_content: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../samples/python/main.py"
        )),
    },
    BuiltinSample {
        template_id: "starter-rust",
        name: "Rust Starter",
        kind: "source",
        language_id: Some("rust"),
        rel_path: "rust/main.rs",
        fallback_content: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../samples/rust/main.rs"
        )),
    },
    BuiltinSample {
        template_id: "starter-java",
        name: "Java Starter",
        kind: "source",
        language_id: Some("java"),
        rel_path: "java/Main.java",
        fallback_content: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../samples/java/Main.java"
        )),
    },
    BuiltinSample {
        template_id: "readme-default",
        name: "Default Problem README",
        kind: "readme",
        language_id: None,
        rel_path: "problem-readme.md",
        fallback_content: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../samples/problem-readme.md"
        )),
    },
];

fn resolve_samples_dir() -> Option<PathBuf> {
    if let Ok(env_dir) = std::env::var("DUCKY_SAMPLES_DIR") {
        let p = PathBuf::from(env_dir);
        if p.is_dir() {
            return Some(p);
        }
    }

    let candidates = [
        PathBuf::from("samples"),
        PathBuf::from("../samples"),
        PathBuf::from("../../samples"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../samples"),
    ];
    candidates.into_iter().find(|p| p.is_dir())
}

fn load_sample_content(sample: &BuiltinSample) -> String {
    if let Some(dir) = resolve_samples_dir() {
        let path = dir.join(sample.rel_path);
        if let Ok(content) = std::fs::read_to_string(path) {
            return content;
        }
    }
    sample.fallback_content.to_string()
}

pub fn load_readme_template() -> String {
    BUILTIN_SAMPLES
        .iter()
        .find(|s| s.template_id == "readme-default")
        .map(load_sample_content)
        .unwrap_or_default()
}

/// Seeds or refreshes built-in templates from `samples/` without overwriting user edits.
pub async fn sync_builtin_templates(pool: &SqlitePool) -> CoreResult<()> {
    for sample in BUILTIN_SAMPLES {
        let content = load_sample_content(sample);
        let existing = sqlx::query(
            "SELECT is_builtin, created_at, updated_at FROM templates WHERE id = ?1",
        )
        .bind(sample.template_id)
        .fetch_optional(pool)
        .await?;

        match existing {
            None => {
                sqlx::query(
                    "INSERT INTO templates (id, name, kind, language_id, content, is_default, is_builtin)
                     VALUES (?1, ?2, ?3, ?4, ?5, 1, 1)",
                )
                .bind(sample.template_id)
                .bind(sample.name)
                .bind(sample.kind)
                .bind(sample.language_id)
                .bind(&content)
                .execute(pool)
                .await?;
            }
            Some(row) => {
                let is_builtin: i64 = row.get(0);
                let created_at: String = row.get(1);
                let updated_at: String = row.get(2);
                if is_builtin == 1 && created_at == updated_at {
                    sqlx::query("UPDATE templates SET content = ?1 WHERE id = ?2")
                        .bind(&content)
                        .bind(sample.template_id)
                        .execute(pool)
                        .await?;
                }
            }
        }
    }
    Ok(())
}
