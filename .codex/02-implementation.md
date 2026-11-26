# Implementation and QA notes

## Boundaries

- `src/core`: no Slint imports; owns persistence, validation and filesystem work.
- `src/state.rs`: owns the SQLite connection and preference reads/writes.
- `src/app.rs`: maps typed domain data into Slint `VecModel` rows and binds callbacks.
- `ui/`: declarative view-only code split by components, pages and dialogs.

## Interaction coverage

- Sidebar navigation; real dashboard/recent models refresh after data changes.
- Create/edit form validates title, URL, identifier, workspace and language selection through the domain service.
- Details supports status update, edit, metadata-only delete with confirmation, opening workspace and source URL.
- Problems support live search, status/difficulty/language/platform/tag filters, and cycling useful sort orders.
- Settings persist theme, EN/VI/system language, default workspace, language and difficulty.
- `Ctrl+N` starts a new problem and `Esc` closes any open form, detail panel, or confirmation layer.

## QA result

On 2026-11-26, `cargo fmt --check`, `cargo check`, `cargo test` (6 passed), and `cargo build --release` passed. The release binary launched with Slint's software renderer on Linux and remained in its native event loop until deliberately interrupted. Screenshot-level visual QA is blocked by this environment's missing computer-use desktop surface; see `03-tauri-migration.md` for exact command and limitation.
