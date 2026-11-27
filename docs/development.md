# Development Guide

## Workspace Layout

- `crates/ducky-core`: Pure domain models, validation, SQLite migrations/repositories, and safe workspace filesystem operations.
- `crates/ducky-server`: Axum HTTP server, security middleware, `rust-embed` asset serving, CLI, and OS shell adapters.
- `frontend`: React + TypeScript + Vite + Tailwind CSS + Hugeicons SPA.

## Development Workflow

Run the backend and Vite hot-reload server together:

```bash
./scripts/dev.sh
```

Or manually in two terminals:

```bash
# Terminal 1
cargo run -p ducky-server -- --no-open --dev-origin http://localhost:5173

# Terminal 2
cd frontend && npm run dev
```

## Testing

```bash
# Backend unit & HTTP integration tests
cargo test --workspace

# Frontend typecheck & production build
cd frontend && npm run check && npm run build
```

## Platform Notes

- **Linux**: Verified directly on this host (`x86_64`, `rustc 1.99.1`, `node v26.13.1`).
- **Windows / macOS**: Designed with portable Rust standard library, `directories`, `open`, and `tokio::signal` abstractions, and wired into `.github/workflows/release.yml`, but unverified directly on non-Linux hosts in this environment.
