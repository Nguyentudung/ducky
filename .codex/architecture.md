# Ducky Architecture

## 1. Overview

Ducky is a single-binary local web application:
1. A **Rust** executable (`ducky-server`) starts an **Axum** HTTP server bound to `127.0.0.1` (default port `8090`).
2. The production **React + TypeScript + Vite** frontend is compiled to static assets and embedded directly into the Rust binary via `rust-embed`.
3. On startup, Ducky initializes SQLite (`sqlx`), prepares the workspace/settings, binds the loopback HTTP listener, prints concise startup status, and launches the user's default browser (unless `--no-open` / `DUCKY_NO_OPEN=1`).
4. End users require **zero runtime dependencies** (no Node.js, npm, Rust, Docker, or external DB).

## 2. Repository Structure

```text
ducky/
├── Cargo.toml                      # Workspace: crates/ducky-core, crates/ducky-server
├── Cargo.lock
├── LICENSE
├── README.md
├── .gitignore
├── .github/workflows/release.yml   # Multi-OS release build workflow
├── crates/
│   ├── ducky-core/                 # Domain logic independent of HTTP & UI
│   │   ├── Cargo.toml
│   │   ├── migrations/             # SQLx SQLite migrations
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs            # Typed domain/validation/filesystem/DB errors
│   │       ├── models.rs           # Problem, Language, Template, Settings, Dashboard DTOs
│   │       ├── validation.rs       # Input normalization, identifier/URL/path checks
│   │       ├── db.rs               # SQLite pool setup, legacy-schema migration, migrations
│   │       ├── problems.rs         # Problem CRUD, search/filter/sort, dashboard, recent
│   │       ├── languages.rs        # Language definitions CRUD
│   │       ├── templates.rs        # Source & README templates CRUD + rendering
│   │       ├── settings.rs         # Persistent application & workspace settings
│   │       └── workspace.rs        # Safe workspace validation, folder/file/README generation
│   └── ducky-server/               # HTTP server, REST API, security, static embedding, OS adapters
│       ├── Cargo.toml
│       ├── build.rs                # Verifies frontend/dist exists or guides build
│       └── src/
│           ├── main.rs             # CLI flags, logging, startup banner, graceful shutdown
│           ├── config.rs           # Host/port/data-dir/browser options
│           ├── state.rs            # Shared AppState (SqlitePool, ServerConfig)
│           ├── server.rs           # Router assembly & TCP binding / port conflict handling
│           ├── middleware.rs       # Host header check, Origin/Sec-Fetch-Site CSRF guard, security headers
│           ├── handlers.rs         # Thin REST API handlers delegating to ducky-core
│           ├── static_files.rs     # rust-embed serving with MIME types, cache headers, SPA fallback
│           └── platform.rs         # Cross-platform browser launch, file-manager open, data/workspace dirs
├── frontend/                       # React + TypeScript + Vite + Tailwind CSS + Hugeicons
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── index.html
│   └── src/
│       ├── main.tsx
│       ├── App.tsx
│       ├── styles/index.css        # Design tokens (light + dark themes) & Tailwind v4
│       ├── types/api.ts            # TypeScript contracts mirroring backend DTOs
│       ├── services/api.ts         # Typed fetch client with X-Ducky-Request header & error parsing
│       ├── i18n/                   # EN & VI translations + resolver (system/en/vi)
│       ├── hooks/                  # Theme, i18n, toast, keyboard shortcuts
│       ├── components/             # Reusable UI primitives & layout shell
│       └── pages/                  # Home, Problems, ProblemDetail, Settings
├── scripts/                        # Dev and release build helpers
├── docs/                           # User & developer documentation
└── .codex/                         # Project journal
```

## 3. Core vs Server Separation

- `ducky-core` has **zero** dependencies on Axum, HTTP types, `rust-embed`, or browser/OS UI crates. It owns:
  - Domain models & validation (`ProblemInput`, `ProblemPatch`, `ProblemQuery`, `LanguageInput`, `TemplateInput`, `SettingsPatch`).
  - SQLite persistence via `sqlx::SqlitePool` and SQL migrations.
  - Safe filesystem operations: workspace root verification, path traversal/symlink guards, atomic non-overwriting source file creation (`create_new(true)`), `README.md` rendering, adding languages/files to existing problems, and scanning generated files inside a problem workspace.
- `ducky-server` owns:
  - CLI parsing (`--port`, `--host`, `--no-open`, `--data-dir`, `--workspace`).
  - Loopback TCP listener (`127.0.0.1:8090` default) with clear `AddrInUse` error reporting.
  - Axum router + REST JSON handlers.
  - Security middleware (`Host`, `Origin`, `Sec-Fetch-Site`, `X-Ducky-Request`, CSP/frame headers).
  - Embedded static asset serving (`rust-embed` + `mime_guess`).
  - OS adapters (`open` crate for browser launch and opening validated workspace folders).

## 4. Database Design (SQLite via SQLx)

Tables:
1. `app_settings (key TEXT PRIMARY KEY, value TEXT NOT NULL)`
2. `languages (id, name, extension, default_filename, folder_name, is_builtin, enabled, sort_order, created_at, updated_at)`
3. `templates (id, name, kind, language_id, content, is_default, is_builtin, created_at, updated_at)`
   - `kind`: `'source'` or `'readme'`
4. `problems (id, identifier, title, description, platform, url, difficulty, tags, status, languages, workspace_path, folder_name, notes, created_at, updated_at, last_opened_at)`
5. `recent_problems (problem_id PRIMARY KEY REFERENCES problems(id) ON DELETE CASCADE, opened_at)`

Legacy migration:
- Before running SQLx migrations, `ducky-core::db::init_pool` checks if `ducky.db` contains the legacy Tauri schema (`problems` with `status_key` column and no `_sqlx_migrations` table). If found, it safely copies existing `problems`, `recent_problems`, `language_definitions`, and `app_settings` into the new schema inside a single transaction, preserving all user data.

Source code files are **never** stored in SQLite; they live as real files in the user's workspace directory.

## 5. Filesystem & Workspace Design

Configured workspace root (e.g. `~/Documents/Ducky` or user-chosen path):
- Optional platform grouping (`group_by_platform`, e.g., `workspace/Codeforces/Two-Sum/` or `workspace/Two-Sum/`).
- Configurable file layout (`file_layout`: `'auto'` | `'subdirs'` | `'flat'`):
  - `'subdirs'`: `<problem_dir>/<lang_folder>/<filename>` (e.g. `cpp/main.cpp`, `rust/main.rs`)
  - `'flat'`: `<problem_dir>/<filename>` (if single or non-colliding; collisions fall back to `<lang_folder>/<filename>` or error before overwriting)
  - `'auto'`: `'flat'` for 1 language, `'subdirs'` for 2+ languages.
- Generated files:
  - `README.md` (rendered from active README template with `{{title}}`, `{{identifier}}`, `{{platform}}`, `{{url}}`, `{{difficulty}}`, `{{status}}`, `{{tags}}`, `{{languages}}`, `{{description}}`, `{{notes}}`, `{{created_at}}`).
  - Language source file(s) from selected template (`starter`, `empty`, or custom template ID).
- Safety guarantees:
  - Reject relative workspace roots, null bytes, `..` path components, and symlinks pointing outside workspace root.
  - Create problem files with `OpenOptions::create_new(true)` so existing files are never silently overwritten.
  - Problem deletion removes the DB record and leaves workspace files intact by default (optional explicit `delete_files=true` only when path is safely inside workspace root and confirmed by user).

## 6. REST API Contract

All API endpoints live under `/api` and return consistent JSON (`{ ... }` on success, `{ "error": { "code": "...", "message": "..." } }` on failure).

- `GET /api/health` — server health, version, bind address, DB status.
- `GET /api/dashboard` — counts, completion %, recent problems, recently modified problems, workspace status.
- `GET /api/problems` — query params: `search`, `status`, `difficulty`, `language`, `platform`, `tag`, `sort`, `dir`.
- `POST /api/problems` — create problem record + workspace folder + source files + `README.md`.
- `GET /api/problems/:id` — problem details + discovered files in problem folder (`files`) + `record_open` option (`?open=1`).
- `PATCH /api/problems/:id` — update metadata/status/notes/tags/languages.
- `DELETE /api/problems/:id` — delete metadata (`?delete_files=true` optionally removes folder inside workspace root).
- `POST /api/problems/:id/files` — add a new source file (language + optional custom filename + template) to an existing problem without overwriting.
- `POST /api/problems/:id/readme` — generate/regenerate `README.md` (`{ "overwrite": bool }`).
- `POST /api/problems/:id/open-folder` — open the validated problem folder in OS file manager.
- `DELETE /api/recent` — clear recent activity history.
- `GET /api/languages` / `POST /api/languages` / `PATCH /api/languages/:id` / `DELETE /api/languages/:id`
- `GET /api/templates` / `POST /api/templates` / `PATCH /api/templates/:id` / `DELETE /api/templates/:id`
- `GET /api/settings` / `PATCH /api/settings`
- `GET /api/workspace` — current workspace status, stats, path validation.
- `POST /api/workspace/validate` — validate candidate workspace path without saving.
- `POST /api/workspace/open` — open configured workspace root in OS file manager.

## 7. Security Model

- Loopback-only default bind (`127.0.0.1`).
- Strict `Host` header validation (`127.0.0.1:<port>`, `localhost:<port>`, `[::1]:<port>`).
- CSRF / cross-origin protection:
  - No wildcard CORS headers.
  - Reject any request whose `Origin` header does not match the local server origin (or explicit dev origin in debug builds).
  - Reject state-changing (`POST`, `PUT`, `PATCH`, `DELETE`) `/api/*` requests if `Sec-Fetch-Site` is `cross-site` or if neither a matching `Origin` nor `X-Ducky-Request: 1` is present.
- Content-Security-Policy & security headers (`X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`).
- No arbitrary filesystem read/write or open endpoints; folder opening is constrained to stored workspace/problem directories validated against traversal and symlinks.
- No command execution or code compilation/running.

## 8. Build & Distribution

1. Build frontend: `cd frontend && npm ci && npm run build` -> `frontend/dist/`.
2. Build Rust release binary: `cargo build --release` -> embeds `frontend/dist/` via `rust-embed`.
3. Cross-platform GitHub Actions workflow builds native artifacts for Linux (`x86_64-unknown-linux-gnu`), Windows (`x86_64-pc-windows-msvc`), and macOS (`aarch64-apple-darwin`, `x86_64-apple-darwin`).
