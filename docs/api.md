# Ducky REST API

Base URL: `http://127.0.0.1:8090/api`

All state-changing requests (`POST`, `PUT`, `PATCH`, `DELETE`) require either a same-origin `Origin` header or `X-Ducky-Request: 1`.

Error responses share a consistent envelope:

```json
{
  "error": {
    "code": "validation_error",
    "message": "Problem title is required"
  }
}
```

## Endpoints

- `GET /api/health` — Server status, version, bind host/port, SQLite connectivity.
- `GET /api/dashboard` — Aggregate counts (`total`, `solved`, `in_progress`, `not_started`, `review`, `completion_percent`), `recent`, `modified`, `platforms`, `tags`, and current `workspace` status.
- `GET /api/problems` — Query parameters: `search`, `status`, `difficulty`, `language`, `platform`, `tag`, `sort` (`modified|created|title|difficulty|status`), `dir` (`asc|desc`).
- `POST /api/problems` — Create problem metadata and generate workspace folder + `README.md` + source files.
- `GET /api/problems/:id` — Problem details and discovered workspace files (`?open=1` records recent activity).
- `PATCH /api/problems/:id` — Partial update of problem metadata/status/notes.
- `DELETE /api/problems/:id` — Delete problem record (`?delete_files=true` also deletes the problem folder if safely within workspace root).
- `POST /api/problems/:id/files` — Create an additional language source file inside the problem folder without overwriting existing files.
- `POST /api/problems/:id/readme` — Generate or regenerate `README.md` (`{ "overwrite": boolean, "template_id"?: string }`).
- `POST /api/problems/:id/open-folder` — Open the validated problem directory in the OS file manager.
- `DELETE /api/recent` — Clear recent-problem history.
- `GET /api/languages`, `POST /api/languages`, `PATCH /api/languages/:id`, `DELETE /api/languages/:id`
- `GET /api/templates`, `POST /api/templates`, `PATCH /api/templates/:id`, `DELETE /api/templates/:id`
- `GET /api/settings`, `PATCH /api/settings`
- `GET /api/workspace`, `POST /api/workspace/validate`, `POST /api/workspace/open`
