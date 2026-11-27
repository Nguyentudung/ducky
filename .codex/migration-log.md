# Migration Log — Clean Slate Architecture Rebuild

Date: 2026-11-27

## Summary of Changes

Rebuilt Ducky from a placeholder/stub Rust crate (`src/main.rs` Hello World + legacy `.ftl` localization files) into a clean, single-binary **Rust local server (`Axum` + `SQLx`/`SQLite`) with an embedded `React` + `TypeScript` + `Vite` + `Tailwind CSS` + `Hugeicons` web UI**.

## Legacy Components Removed

- Removed root stub `src/main.rs` and root single-package manifest.
- Removed legacy Fluent runtime localization files `extensions/locales/{en,vi}/app.ftl` after migrating English and Vietnamese catalogs into `frontend/src/i18n/translations.ts`.
- Verified no Slint, GTK, libadwaita, or Tauri code or dependencies remain in the repository.

## New Components Implemented

1. **`crates/ducky-core`**:
   - Typed domain models and validation (`models.rs`, `validation.rs`, `error.rs`).
   - SQLite persistence via `sqlx` (`migrations/0001_init.sql`, `db.rs`) including transactional migration from legacy Tauri/Slint `ducky.db` schemas without deleting user data.
   - Problem CRUD, search, filtering, sorting, dashboard stats, and recent history (`problems.rs`).
   - Configurable programming languages and source/README templates (`languages.rs`, `templates.rs`).
   - Persistent settings (`settings.rs`) and safe, non-overwriting atomic workspace/folder/file/README management (`workspace.rs`).
2. **`crates/ducky-server`**:
   - Loopback-bound Axum HTTP server (`127.0.0.1:8090` default), CLI flags, graceful SIGINT/SIGTERM shutdown, port-conflict error reporting, and OS browser/file-manager integration (`main.rs`, `config.rs`, `platform.rs`, `server.rs`, `state.rs`).
   - REST JSON API (`handlers.rs`) and security middleware (`middleware.rs`) blocking DNS rebinding (`Host` check) and CSRF (`Origin`, `Sec-Fetch-Site`, `X-Ducky-Request: 1`, CSP).
   - Embedded frontend serving (`static_files.rs`) via `rust-embed` + `mime_guess` with cache headers and SPA fallback.
3. **`frontend/`**:
   - Design system (`src/styles/index.css`, `src/components/ui.tsx`) with dedicated Light & Dark palettes, Hugeicons (`src/components/Icon.tsx`), bilingual EN/VI support (`src/i18n/translations.ts`), toast notifications, and responsive shell (`src/components/Layout.tsx`).
   - Pages: `HomePage.tsx`, `ProblemsPage.tsx`, `ProblemDetailPage.tsx`, `SettingsPage.tsx`, plus `ProblemFormModal.tsx` with progressive disclosure.
4. **Build, CI & Documentation**:
   - `scripts/build.sh`, `scripts/dev.sh`, `.github/workflows/release.yml`.
   - `README.md`, `docs/api.md`, `docs/development.md`, `docs/security.md`.
