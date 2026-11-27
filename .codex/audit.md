# Repository Audit — Ducky Clean Slate Rebuild

Date: 2026-11-27

## 1. Repository & Git State

- **Branch**: `main` (clean working tree, synced with `origin/main` at `573a898`).
- **Recent commit history**:
  - `f0c29cc` (`v0.0.1`): GTK4 / libadwaita prototype (`src/application.rs`, `src/widgets/`).
  - `071e850` (`v0.0.4`): Slint 1.19.1 + `rusqlite` + `fluent-bundle` + `rfd` + `open` desktop app, plus earlier Tauri-compatible SQLite schema notes.
  - `66d3ffb` / `573a898` (`edit: project restructuring`): Removed the Slint UI (`ui/*.slint`), `build.rs`, `src/app.rs`, `src/core/`, `src/state.rs`, `src/platform/`, `docs/*.md`, `README.md`, and `.codex/*` prior notes, leaving a minimal placeholder `Cargo.toml` and `src/main.rs` (`println!("Hello, world!");`).

## 2. Current Files in Repository

| Path | Role | Status / Disposition |
|---|---|---|
| `Cargo.toml`, `Cargo.lock` | Minimal stub package (`ducky` v0.0.4, no deps) | Replace with Cargo workspace (`crates/ducky-core`, `crates/ducky-server`) |
| `src/main.rs` | Hello-world placeholder | Remove after moving entry point to `crates/ducky-server/src/main.rs` |
| `extensions/locales/en/app.ftl` | Fluent English UI strings (140 lines) | Migrate EN translations to new frontend i18n module, then remove `extensions/` |
| `extensions/locales/vi/app.ftl` | Fluent Vietnamese UI strings (140 lines) | Migrate VI translations to new frontend i18n module, then remove `extensions/` |
| `LICENSE` | MIT License (2026 Nguyễn Tư Dũng) | Preserve untouched |
| `.gitignore` | Basic Rust ignore file | Update for workspace, `frontend/node_modules`, `frontend/dist`, test artifacts |
| `README.md`, `docs/`, `.codex/` | Empty placeholders | Populate with accurate new documentation and project journal |

## 3. User Data & Environment Findings (Must Preserve)

Inspecting the host environment read-only revealed existing user artifacts outside the repository that must **not** be deleted or broken:
- **SQLite database**: `~/.local/share/ducky/ducky.db`
  - Contains legacy Tauri-era tables (`user_profile`, `app_settings`, `problem_statuses`, `language_definitions`, `problems`, `problem_languages`, `recent_problems`).
  - `problems` currently has 0 rows.
  - `app_settings` contains `workspace_path` (`/home/nguyen-tu-dung/Documents/learn-c`), `theme` (`dark`), `app_language`/`language` (`vi`), and `readme_template`.
  - `language_definitions` defines `c` (`main.c`), `cpp` (`main.cpp`), `rust` (`main.rs`), `python` (`solution.py`), and `java` (`Main.java`).
- **User workspace**: `~/Documents/learn-c/` containing existing practice folders (`C01030 — PHÂN TÍCH THỪA SỐ NGUYÊN TỐ 1/`, `C03001/`).

**Compatibility decision**:
- Use the standard platform application data directory (`ProjectDirs::from("com", "Ducky", "Ducky")` -> `~/.local/share/ducky/ducky.db` on Linux, overridable via `DUCKY_DATA_DIR` / `--data-dir` for testing/isolation).
- When opening an existing `ducky.db` with the legacy Tauri schema, safely migrate existing `app_settings`, `language_definitions`, `problems`, and `recent_problems` into the new SQLx-managed schema in a transaction without losing user settings or problems.

## 4. Features Worth Preserving & Improving

From inspecting `v0.0.4` in git history (`071e850`) and the existing `app.ftl` catalogs:
- **Problem tracking**: title, identifier, platform/source, URL, difficulty (`easy`, `medium`, `hard`, `custom`), status (`not_started`, `in_progress`, `solved`, `review`), tags, languages, description, notes, timestamps (`created_at`, `updated_at`, `last_opened_at`).
- **Recent activity**: `recent_problems` table tracking recently opened problems with clear-history support.
- **Dashboard overview**: total, solved, in-progress, not-started, review counts, completion rate, recent & recently modified problems.
- **Workspace & file generation**:
  - Configurable workspace root with strict path validation.
  - Safe problem folder creation (no path traversal, atomic/non-overwriting file creation with `create_new(true)`).
  - Multi-language support (`c`, `cpp`, `rust`, `python`, `java`) plus customizable languages, filenames, folder layout (`language_subdirs` vs `flat`, `group_by_platform`), and customizable templates (source templates + README template).
  - Deleting a problem from Ducky deletes metadata only by default (preserving workspace files on disk unless user explicitly checks workspace deletion in a confirmed dialog — actually keeping files safe by default and making deletion behavior explicit).
- **Preferences & Localization**:
  - Theme: `system`, `light`, `dark`.
  - Language: `system`, `en`, `vi`.
  - Default language, difficulty, status, template, workspace layout.
- **OS integration**: opening the workspace/problem folder in the system file manager and opening problem URLs (or opening directly in browser).

## 5. Legacy Components to Remove

- Stub `src/main.rs` and root `[package]` manifest (replaced by Cargo workspace).
- `extensions/locales/` Fluent `.ftl` runtime files (replaced by clean TypeScript EN/VI i18n bundles in `frontend/`).
- No Slint, GTK, Tauri, or desktop UI crate dependencies will remain.
