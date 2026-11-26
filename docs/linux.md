# Linux development and build

Ducky is a Rust + Slint application. It does not require Node.js, WebKit or a WebView.

## Requirements

- Rust 1.95 or later (the development environment uses Rust 1.99.1)
- Linux X11 or Wayland runtime libraries required by Slint's Winit backend

On Debian/Ubuntu, the Slint documentation lists these X11 development prerequisites when they are missing:

```bash
sudo apt install libx11-xcb-dev xinput libxcursor-dev libxkbcommon-x11-dev libx11-dev
```

## Commands

```bash
cargo fmt --check
cargo check
cargo test
cargo build --release
cargo run
```

The default renderer is FemtoVG. On machines without usable GPU/EGL support, Ducky includes Slint's software renderer:

```bash
SLINT_BACKEND=winit-software cargo run
```

## Data and workspace

- Database: platform app-data directory, `com/Ducky/Ducky/ducky.db`
- Default workspace: `Documents/Ducky` where available
- Existing Tauri databases and problem workspaces remain compatible; migration is additive and does not rewrite source files.
- `DUCKY_WORKSPACE=/absolute/path` is reserved for automated or sandbox QA and is not necessary in normal use.

## Packaging note

The code is intentionally cross-platform: Slint's Winit backend supports Linux, Windows and macOS; `rfd` supplies native folder selection; `open` delegates folder/URL opening to the host OS. This migration was built and launched only on Linux. Windows and macOS still require their own release/installer validation.
