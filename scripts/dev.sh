#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "==> Starting Rust backend on 127.0.0.1:8090 (dev origin http://localhost:5173)..."
(cd "$ROOT_DIR" && cargo run -p ducky-server -- --no-open --dev-origin http://localhost:5173) &
SERVER_PID=$!

echo "==> Starting Vite dev server on http://localhost:5173..."
(cd "$ROOT_DIR/frontend" && npm run dev)
