# Ducky

**Make coding feel easier.**

Ducky is a lightweight, single-binary programming practice workspace manager. It starts a local Rust HTTP server, serves an embedded web interface, opens your browser automatically, and keeps your problems, starter files, and notes organized on disk without trying to be an IDE.

## Highlights

- **Zero runtime dependencies**: End users only run one executable—no Node.js, npm, Rust, Docker, or external database required.
- **Real workspace files**: Source files (`C`, `C++`, `Python`, `Rust`, `Java`, plus custom languages) and `README.md` live directly in your chosen workspace folder.
- **SQLite metadata**: Problem metadata, statuses, recent history, languages, templates, and preferences persist in SQLite via SQLx.
- **Safe filesystem operations**: Atomic folder creation, non-overwriting file creation, path-traversal and symlink-escape protection, and non-destructive metadata deletion by default.
- **Clean local UI**: Light, Dark, and System themes; English and Vietnamese interface; fast filtering, search, sorting, and progressive-disclosure problem creation.

## Running Ducky

```bash
./ducky
```

By default, Ducky binds to `http://127.0.0.1:8090` and opens your default browser.

### CLI Flags & Environment Variables

| Flag | Environment Variable | Default | Description |
|---|---|---|---|
| `--host` | `DUCKY_HOST` | `127.0.0.1` | Bind address (loopback recommended) |
| `--port` / `-p` | `DUCKY_PORT` | `8090` | Listening port |
| `--no-open` | `DUCKY_NO_OPEN` | `false` | Do not auto-launch the browser |
| `--data-dir` | `DUCKY_DATA_DIR` | OS app data dir | Directory for `ducky.db` |
| `--workspace` | `DUCKY_WORKSPACE` | `~/Documents/Ducky` | Default problem workspace root |

## Workspace Structure

Depending on your workspace settings (`group_by_platform` and `file_layout`), Ducky generates structures like:

```text
workspace/
├── Codeforces/
│   ├── Two-Sum/
│   │   ├── README.md
│   │   ├── cpp/
│   │   │   └── main.cpp
│   │   └── rust/
│   │       └── main.rs
│   └── Problem-B/
│       ├── README.md
│       └── main.cpp
└── LeetCode/
    └── Binary-Search/
        ├── README.md
        └── solution.py
```

## Building from Source

Prerequisites (development/build only):
- Rust toolchain (`cargo`, `rustc`)
- Node.js & npm

```bash
# 1. Build the embedded frontend
cd frontend
npm ci
npm run build
cd ..

# 2. Build the release binary (embeds frontend/dist/)
cargo build --release
```

Or run `./scripts/build.sh`.

## Documentation

- [API Contract](docs/api.md)
- [Configuration & Runtime Options](docs/configuration.md)
- [Workspace Behavior & File Generation](docs/workspace.md)
- [Development Guide](docs/development.md)
- [Security & Workspace Safety](docs/security.md)

## License

MIT — see [LICENSE](LICENSE).
