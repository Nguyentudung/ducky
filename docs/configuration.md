# Configuration & Runtime Options

## Server CLI & Environment Variables

| Option | Env Variable | Default | Notes |
|---|---|---|---|
| `--host` | `DUCKY_HOST` | `127.0.0.1` | Loopback recommended; warns if non-loopback |
| `--port`, `-p` | `DUCKY_PORT` | `8090` | Local HTTP port |
| `--no-open` | `DUCKY_NO_OPEN` | `false` | Skip launching default browser |
| `--data-dir` | `DUCKY_DATA_DIR` | OS data dir | Directory holding `ducky.db` |
| `--workspace` | `DUCKY_WORKSPACE` | `~/Documents/Ducky` | Default workspace root override |
| `--dev-origin` | `DUCKY_DEV_ORIGIN` | unset | Extra allowed origin for Vite dev server |

## Persistent Application Settings (`app_settings`)

Editable from the **Settings** page in the UI or via `PATCH /api/settings`:

- `workspace_path`: Active workspace root directory.
- `theme`: `system` | `light` | `dark`.
- `language`: `system` | `en` | `vi`.
- `default_language`: Pre-selected language ID for new problems (e.g. `cpp`).
- `default_difficulty`: `easy` | `medium` | `hard` | `custom`.
- `default_status`: `not_started` | `in_progress` | `solved` | `review`.
- `default_template_mode`: `starter` | `empty`.
- `group_by_platform`: `true` | `false`.
- `file_layout`: `auto` | `subdirs` | `flat`.
- `generate_readme`: `true` | `false`.

## README Template Placeholders

`{{title}}`, `{{identifier}}`, `{{platform}}` (or `{{source}}`), `{{url}}`, `{{difficulty}}`, `{{status}}`, `{{tags}}`, `{{languages}}`, `{{description}}` (or `{{statement}}`), `{{notes}}`, `{{created_at}}`, `{{updated_at}}`.
