# Migration journal — Tauri/React to Slint

## Audit (2026-11-26)

The repository contained three overlapping products:

1. A legacy GTK/libadwaita root crate with mostly placeholder core modules.
2. The functional Tauri 2 application in `src-tauri/`, including SQLite problem CRUD, safe workspaces, Fluent localization, and a Vite/TypeScript UI.
3. An uncommitted frontend/UI redesign that was complete enough to use as feature and content reference.

The reusable source of truth was the Tauri Rust core, not the GTK code. Its SQLite database is located at `com/Ducky/Ducky/ducky.db`; the Slint app intentionally keeps that path, table names and columns unchanged. Workspaces and user-authored source files are never moved or deleted by migration.

## Research decisions

- Slint **1.19.1** was selected after checking its official release and current Rust docs.
- `build.rs` compiles modular external `.slint` files. `VecModel`/`ModelRc` move typed list rows into the UI; callbacks send user intent back to Rust.
- Core services remain independent of Slint. `src/app.rs` is the controller boundary; it owns callbacks and only projects domain data into UI models.
- Slint's Winit backend provides Linux/Windows/macOS compatibility. FemtoVG is the default renderer and software rendering is compiled as a GPU-less fallback (`SLINT_BACKEND=winit-software`).
- `rfd` provides native directory selection and `open` delegates URL/folder integration to the OS.
- Theme values live in `ui/theme/tokens.slint`, not scattered in pages. Light and dark palettes are designed separately; system mode uses `dark-light` at launch/change time.

## New architecture

```
src/
  app.rs                 Slint controller and callback binding
  state.rs               SQLite/app preferences boundary
  core/                  Slint-independent database, i18n, problems, workspace
  platform/              Cross-platform folder/URL operations
ui/
  app.slint              Application shell
  theme/tokens.slint     Central design tokens
  components/            Controls and problem rows
  pages/                 Home, Problems, Recent and Settings
  dialogs.slint          Create/edit, details, confirmation dialogs
```

## Feature migration status

Migrated and implemented: persisted preferences, Fluent EN/VI strings, SQLite schema compatibility, problem CRUD, search, filters, sorting, dashboard aggregates, recent history, source templates, safe workspace generation, workspace settings, themes, external open operations, and all four main pages.

The old Tauri runtime, plugins, Vite, TypeScript frontend and GTK app have been removed. No web build chain remains.

## Verification log

- `cargo fmt --check`, `cargo check`, and `cargo test` — passed: 6 tests (workspace generation/no-overwrite and path validation; EN/VI switching; empty dashboard; problem status/recent persistence; settings replacement).
- `CARGO_TARGET_DIR=/tmp/ducky-release-target cargo build --release` — passed.
- `SLINT_BACKEND=winit-software XDG_DATA_HOME=/tmp/ducky-visual-data DUCKY_WORKSPACE=/tmp/ducky-visual-workspace /tmp/ducky-release-target/release/ducky` — launched successfully and kept the native event loop alive until manually interrupted. A previous empty-dashboard launch exposed a SQLite `SUM()`-on-empty-set bug; fixed using `COALESCE`.
- The available computer-use host exposed no desktop applications or screenshot surface, so visual inspection/screenshot QA could not be completed here. This is a known environment limitation, not represented as completed QA.

## Known limitations

- UI manual visual QA, native picker behaviour, and external opener behaviour need a normal desktop session with accessible computer-use surface.
- Windows and macOS are designed for, but not tested in this Linux environment.
- Workspace relocation remains deliberately explicit: changing the default applies to future problems and never moves old folders.
- Pagination/virtualization for exceptionally large collections is future work; normal filtering is database-backed.
