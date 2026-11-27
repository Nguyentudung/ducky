# Testing & Verification Log

Date: 2026-11-27

## 1. Backend Unit & Integration Tests

Command: `cargo test --workspace`

- **`ducky-core` (4/4 passed)**:
  - `tests::rejects_path_traversal_and_reserved_names` — verified rejection of `..`, `/`, `\`, and Windows reserved names (`CON`).
  - `tests::full_problem_workspace_lifecycle` — verified empty dashboard, problem creation with multi-language folder layout + `README.md`, duplicate identifier rejection, adding custom-named language files without overwriting, README overwrite protection + explicit overwrite, status/notes updates, recent-activity tracking + clearing, search/filtering, and non-destructive problem deletion.
  - `tests::settings_languages_and_templates_persist` — verified settings persistence (`theme`, `language`, `group_by_platform`, `file_layout`), built-in language protection, custom language CRUD, custom template CRUD, and workspace path validation.
  - `tests::migrates_legacy_tauri_database_without_losing_settings` — verified automatic migration of a legacy Tauri-era SQLite database preserving `app_settings`, `readme_template`, and `problems`.
- **`ducky-server` (3/3 passed)**:
  - `server::tests::health_and_static_routes_work` — verified `/api/health`, `/api/*` JSON 404 handling, and SPA route fallback (`/problems/42`).
  - `server::tests::security_guard_blocks_dns_rebinding_and_csrf` — verified `403 Forbidden` for foreign `Host` header, foreign `Origin` header, and missing `Origin` / `X-Ducky-Request` on state-changing requests.
  - `server::tests::api_problem_crud_and_settings_flow` — verified end-to-end REST API `POST /api/problems`, `PATCH /api/problems/:id`, and `DELETE /api/problems/:id`.

## 2. Frontend Typecheck & Production Build

Command: `cd frontend && npm run build` (`tsc -b && vite build`)

- TypeScript strict compilation (`tsc -b`): **0 errors**.
- Production Vite bundle (`frontend/dist/`):
  - `index.html`: `0.49 kB` (`0.32 kB` gzip)
  - `assets/index-*.css`: `22.96 kB` (`5.28 kB` gzip)
  - `assets/index-*.js`: `351.52 kB` (`103.03 kB` gzip)

## 3. Release Binary & Live Runtime Verification

Command: `cargo build --release`

- Output binary: `target/release/ducky` (**5.5 MB**, embeds `frontend/dist` via `rust-embed`).
- **Startup & Port Conflict**: Launched `./target/release/ducky --no-open --port 18090` with isolated `--data-dir` and `--workspace`; verified clean startup banner and verified a second instance on port `18090` exited with a clear `Port 18090 is already in use` message.
- **API & Workspace Generation**: Created a problem via `POST /api/problems` and verified `LeetCode/Two Sum/README.md`, `cpp/main.cpp`, and `python/solution.py` were created on disk.
- **Persistence & Graceful Shutdown**: Sent `SIGINT`, confirmed `Ducky stopped cleanly.`, restarted on the same data directory, and confirmed `GET /api/problems` returned the persisted problem.
- **Measured Backend Process Resource Usage** (`ps -o pid,rss,vsz,comm`):
  - Idle `ducky` Rust server RSS: **8,104 KB (~7.9 MB)**.

## 4. Visual & Interactive Browser QA (Headless Chrome 158)

Executed live interactive browser QA via Chrome DevTools Protocol against `./target/release/ducky` on `127.0.0.1:18091`:
- Verified **0 console errors** and **0 uncaught runtime exceptions**.
- Visually inspected rendered screenshots across:
  - Light theme: Empty Home, Create Problem modal (with pinned footer and expanded Advanced & File Generation options), Problem Detail page, Problems list.
  - Dark theme: Settings Appearance tab, Home overview with progress & recent/modified lists, Problems list.
  - Mobile (`390x844`): Responsive top bar, filter stack, problem cards, and toast alignment.

## 5. Platform Scope

- Verified on **Linux (`x86_64`)**.
- **Windows** and **macOS** targets are configured in `.github/workflows/release.yml` using cross-platform Rust APIs, but remain unverified on native Windows/macOS hosts in this environment.
