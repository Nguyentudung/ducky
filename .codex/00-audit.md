# Repository audit — Slint migration

Date: 2026-11-26

## Starting condition

- Root: legacy GTK/libadwaita prototype, mostly placeholder domain code.
- `src-tauri/`: functional Tauri 2 backend with SQLite, workspaces, templates, Fluent i18n and a Vite/TypeScript interface.
- User data: `com/Ducky/Ducky/ducky.db`; generated problem workspaces include `problem.json`, `README.md`, `notes.md`, and `solutions/<language>/`.

## Reused safely

- Database location, tables (`app_settings`, `problems`, `recent_problems`) and columns.
- Input validation, duplicate identifier rules, recent-history semantics, safe workspace and all five template formats.
- Fluent English/Vietnamese catalogs, extended for the native Slint views.

## Removed after replacement

- GTK/libadwaita window/widgets/CSS.
- Tauri runtime, commands, capabilities, config and plugins.
- React/Vite/TypeScript/Node package manifests and built assets.

## Compatibility guarantee

The schema remains additive/idempotent. Changing default workspace never relocates prior folders; deleting a problem removes only Ducky metadata, not a user workspace.
