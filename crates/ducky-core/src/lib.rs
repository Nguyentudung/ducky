pub mod db;
pub mod error;
pub mod languages;
pub mod models;
pub mod problems;
pub mod samples;
pub mod settings;
pub mod templates;
pub mod validation;
pub mod workspace;

#[cfg(test)]
mod tests {
    use crate::db::init_memory_pool;
    use crate::languages::{create_language, delete_language, list_languages};
    use crate::models::{
        AddProblemFileInput, CreateLanguageInput, CreateProblemInput, CreateTemplateInput,
        GenerateReadmeInput, ProblemQuery, SettingsPatch, UpdateProblemInput,
    };
    use crate::problems::{
        add_problem_file, clear_recent, create_problem, dashboard_stats, delete_problem,
        generate_problem_readme, get_problem_detail, list_problems, update_problem,
    };
    use crate::settings::{get_settings, patch_settings};
    use crate::templates::{create_template, delete_template, list_templates};
    use crate::validation::validate_single_component;
    use crate::workspace::{ensure_within_workspace, validate_workspace_path};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(suffix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("ducky-core-test-{suffix}-{nanos}"))
    }

    #[test]
    fn rejects_path_traversal_and_reserved_names() {
        assert!(validate_single_component("../escape", "Folder").is_err());
        assert!(validate_single_component("foo/bar", "File").is_err());
        assert!(validate_single_component("foo\\bar", "File").is_err());
        assert!(validate_single_component("CON", "File").is_err());
        assert!(validate_single_component("main.cpp", "File").is_ok());
    }

    #[tokio::test]
    async fn full_problem_workspace_lifecycle() {
        let ws = temp_root("lifecycle");
        std::fs::create_dir_all(&ws).unwrap();
        let pool = init_memory_pool().await.unwrap();

        let empty_dash = dashboard_stats(&pool).await.unwrap();
        assert_eq!(empty_dash.total, 0);

        let created = create_problem(
            &pool,
            &ws,
            CreateProblemInput {
                identifier: "cf-100a".into(),
                title: "Two Sum".into(),
                platform: "Codeforces".into(),
                difficulty: "easy".into(),
                tags: vec!["arrays".into(), "hash-map".into()],
                languages: vec!["cpp".into(), "rust".into()],
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(created.problem.title, "Two Sum");
        assert!(created.workspace_exists);
        let pdir = PathBuf::from(&created.problem.workspace_path);
        assert!(pdir.join("README.md").is_file());
        assert!(pdir.join("cpp/main.cpp").is_file());
        assert!(pdir.join("rust/main.rs").is_file());

        // Duplicate identifier or folder name must fail cleanly without overwriting
        let dup = create_problem(
            &pool,
            &ws,
            CreateProblemInput {
                identifier: "cf-100a".into(),
                title: "Two Sum Duplicate".into(),
                platform: "Codeforces".into(),
                languages: vec!["cpp".into()],
                ..Default::default()
            },
        )
        .await;
        assert!(dup.is_err());

        // Add Python file without overwriting
        let added = add_problem_file(
            &pool,
            &ws,
            created.problem.id,
            AddProblemFileInput {
                language_id: "python".into(),
                filename: Some("custom.py".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(added.name, "custom.py");
        assert!(pdir.join("python/custom.py").is_file());

        // Cannot overwrite README unless overwrite=true
        assert!(
            generate_problem_readme(
                &pool,
                created.problem.id,
                GenerateReadmeInput {
                    overwrite: false,
                    template_id: None
                }
            )
            .await
            .is_err()
        );
        generate_problem_readme(
            &pool,
            created.problem.id,
            GenerateReadmeInput {
                overwrite: true,
                template_id: None,
            },
        )
        .await
        .unwrap();

        // Update status & open detail to record recent
        let updated = update_problem(
            &pool,
            created.problem.id,
            UpdateProblemInput {
                status: Some("solved".into()),
                notes: Some("O(n) hash map".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(updated.status, "solved");

        let detail = get_problem_detail(&pool, created.problem.id, true)
            .await
            .unwrap();
        assert!(detail.problem.last_opened_at.is_some());

        let dash = dashboard_stats(&pool).await.unwrap();
        assert_eq!(dash.total, 1);
        assert_eq!(dash.solved, 1);
        assert_eq!(dash.recent.len(), 1);

        let filtered = list_problems(
            &pool,
            ProblemQuery {
                search: "Two Sum".into(),
                status: "solved".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(filtered.len(), 1);

        clear_recent(&pool).await.unwrap();
        assert!(dashboard_stats(&pool).await.unwrap().recent.is_empty());

        // Default delete keeps files on disk
        delete_problem(&pool, &ws, created.problem.id, false)
            .await
            .unwrap();
        assert!(pdir.is_dir());

        std::fs::remove_dir_all(&ws).unwrap();
    }

    #[tokio::test]
    async fn settings_languages_and_templates_persist() {
        let ws = temp_root("settings");
        std::fs::create_dir_all(&ws).unwrap();
        let pool = init_memory_pool().await.unwrap();

        let initial = get_settings(&pool, &ws).await.unwrap();
        assert_eq!(initial.theme, "system");

        let patched = patch_settings(
            &pool,
            &ws,
            SettingsPatch {
                theme: Some("dark".into()),
                language: Some("vi".into()),
                group_by_platform: Some(false),
                file_layout: Some("flat".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(patched.theme, "dark");
        assert_eq!(patched.language, "vi");
        assert!(!patched.group_by_platform);
        assert_eq!(patched.file_layout, "flat");

        let langs = list_languages(&pool).await.unwrap();
        assert!(langs.len() >= 5);
        assert!(delete_language(&pool, "cpp").await.is_err());

        let go = create_language(
            &pool,
            CreateLanguageInput {
                id: "go".into(),
                name: "Go".into(),
                extension: "go".into(),
                default_filename: "main.go".into(),
                folder_name: "go".into(),
            },
        )
        .await
        .unwrap();
        assert_eq!(go.id, "go");
        delete_language(&pool, "go").await.unwrap();

        let tpl = create_template(
            &pool,
            CreateTemplateInput {
                name: "Fast C++".into(),
                kind: "source".into(),
                language_id: Some("cpp".into()),
                content: "// fast".into(),
                is_default: false,
            },
        )
        .await
        .unwrap();
        assert!(
            list_templates(&pool)
                .await
                .unwrap()
                .iter()
                .any(|t| t.id == tpl.id)
        );
        delete_template(&pool, &tpl.id).await.unwrap();

        let ws_status = validate_workspace_path(&ws);
        assert!(ws_status.valid);
        assert!(ensure_within_workspace(&ws, &ws).is_ok());

        std::fs::remove_dir_all(&ws).unwrap();
    }

    #[tokio::test]
    async fn migrates_legacy_tauri_database_without_losing_settings() {
        let dir = temp_root("legacy-db");
        std::fs::create_dir_all(&dir).unwrap();
        let db_path = dir.join("ducky.db");

        // Create legacy Tauri-era tables
        {
            let pool =
                sqlx::SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path.display()))
                    .await
                    .unwrap();
            sqlx::query("CREATE TABLE app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL)")
                .execute(&pool)
                .await
                .unwrap();
            sqlx::query("INSERT INTO app_settings (key, value) VALUES ('theme', 'dark'), ('app_language', 'vi'), ('readme_template', '# Custom {{title}}')")
                .execute(&pool)
                .await
                .unwrap();
            sqlx::query("CREATE TABLE problems (id INTEGER PRIMARY KEY, title TEXT NOT NULL, folder_name TEXT NOT NULL UNIQUE, source TEXT NOT NULL DEFAULT '', url TEXT NOT NULL DEFAULT '', status_key TEXT NOT NULL, statement TEXT NOT NULL DEFAULT '', notes TEXT NOT NULL DEFAULT '', created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)")
                .execute(&pool)
                .await
                .unwrap();
            sqlx::query("INSERT INTO problems (id, title, folder_name, source, status_key) VALUES (1, 'Legacy Task', 'legacy-task', 'Codeforces', 'solved')")
                .execute(&pool)
                .await
                .unwrap();
            pool.close().await;
        }

        let migrated_pool = crate::db::init_pool(&db_path).await.unwrap();
        let settings = get_settings(&migrated_pool, &dir).await.unwrap();
        assert_eq!(settings.theme, "dark");
        assert_eq!(settings.language, "vi");

        let problems = list_problems(&migrated_pool, ProblemQuery::default())
            .await
            .unwrap();
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].title, "Legacy Task");
        assert_eq!(problems[0].platform, "Codeforces");
        assert_eq!(problems[0].status, "solved");

        let templates = list_templates(&migrated_pool).await.unwrap();
        let readme = templates.iter().find(|t| t.id == "readme-default").unwrap();
        assert_eq!(readme.content, "# Custom {{title}}");

        migrated_pool.close().await;
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test]
    async fn loads_minimal_boilerplate_samples_from_samples_directory() {
        let pool = init_memory_pool().await.unwrap();
        let templates = list_templates(&pool).await.unwrap();
        for (id, expected_marker) in [
            ("starter-c", "int main(void)"),
            ("starter-cpp", "sync_with_stdio"),
            ("starter-python", "def solve()"),
            ("starter-rust", "fn main()"),
            ("starter-java", "public class Main"),
        ] {
            let tpl = templates
                .iter()
                .find(|t| t.id == id)
                .unwrap_or_else(|| panic!("missing sample template {id}"));
            assert!(
                tpl.content.contains(expected_marker),
                "template {id} did not come from samples/: {}",
                tpl.content
            );
            // Must be minimal boilerplate without pre-written algorithms
            assert!(
                !tpl.content.contains("sort(")
                    && !tpl.content.contains("lower_bound")
                    && !tpl.content.contains("bisect_left"),
                "template {id} should not contain pre-written algorithms: {}",
                tpl.content
            );
        }
    }
}
