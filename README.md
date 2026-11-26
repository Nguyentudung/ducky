# Ducky

Ducky is a lightweight, native desktop companion for programming practice. It keeps problem metadata in SQLite and gives every problem a safe, portable workspace with starter files.

## Stack

| Technology | Role |
|---|---|
| Rust | Application, domain services, persistence and platform integration |
| Slint 1.19 | Native declarative desktop UI |
| SQLite / rusqlite | Durable local settings, problems and recent history |
| Fluent | English and Vietnamese localization |

## Features

- Home dashboard backed by real problem and recent-history data
- Problem CRUD, filtering, search, sort, status tracking and details
- Safe per-problem workspace creation and C/C++/Rust/Python/Java starter templates
- Persistent workspace, language, theme and default problem preferences
- Light, dark and system appearance options
- Native folder selection and operating-system URL/folder opening

## Development

```bash
cargo check
cargo test
cargo run
```

The application stores its database at the platform application-data location (`com/Ducky/Ducky/ducky.db`), deliberately preserving the location and schema used by the prior Tauri release.

For sandbox or CI only, an absolute `DUCKY_WORKSPACE` value overrides the normal Documents/Ducky default. To force the portable CPU renderer, use `SLINT_BACKEND=winit-software cargo run`.

See [Linux development notes](docs/linux.md) and [research references](docs/references.md).

## License

MIT
