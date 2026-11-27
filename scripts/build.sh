#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building frontend..."
(cd "$ROOT_DIR/frontend" && npm ci && npm run build)

echo "==> Building release binary..."
(cd "$ROOT_DIR" && cargo build --release)

echo "==> Done: $ROOT_DIR/target/release/ducky"
