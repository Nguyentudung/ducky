# Ducky

> A programming practice manager built with Rust and GTK 4.

## Technology Stack

| Technology | Purpose |
|---|---|
| Rust | Main programming language |
| GTK 4 | Native desktop GUI toolkit |
| gtk4-rs | Rust bindings for GTK 4 |
| GLib | Core utilities, object system and event infrastructure |
| GIO | Application infrastructure and I/O |
| Cargo | Dependency management and build system |

---

## Environment Setup

### Ubuntu

Install the required development tools and GTK 4 libraries:

```bash
sudo apt update
sudo apt install build-essential pkg-config libgtk-4-dev
```

Verify GTK 4 installation:

```bash
pkg-config --modversion gtk4
```

Verify Rust installation:

```bash
rustc --version
cargo --version
```

If Rust is not installed, install it through [rustup](https://rustup.rs/).

### Install Rust Dependencies

Add GTK 4 bindings to the project:

```bash
cargo add gtk4
```

Cargo manages Rust crates and dependencies through `Cargo.toml` and `Cargo.lock`.

---

## Running Ducky

Run the application in development mode:

```bash
cargo run
```

Compile without running:

```bash
cargo build
```

---

## Building

### Debug Build

```bash
cargo build
```

Debug executable:

```text
target/debug/ducky
```

### Release Build

Build an optimized release version:

```bash
cargo build --release
```

Release executable:

```text
target/release/ducky
```

Run the release build:

```bash
./target/release/ducky
```

---

## Useful Cargo Commands

| Command | Description |
|---|---|
| `cargo run` | Build and run the application |
| `cargo check` | Check the code without producing an executable |
| `cargo build` | Build debug version |
| `cargo build --release` | Build optimized release version |
| `cargo run --release` | Build and run release version |
| `cargo clean` | Remove build artifacts |
| `cargo update` | Update dependencies within version constraints |
| `cargo tree` | Display dependency tree |
| `cargo fmt` | Format Rust code |
| `cargo clippy` | Run Rust lint checks |

---

## Official Documentation

### Rust

- [Rust Official Book](https://doc.rust-lang.org/book/) — The Rust programming language.
- [Rust Standard Library](https://doc.rust-lang.org/std/) — Standard library API reference.
- [Cargo Book](https://doc.rust-lang.org/cargo/) — Cargo package manager and build system.

### GTK 4

- [GTK 4 Documentation](https://docs.gtk.org/gtk4/) — GTK 4 API reference.
- [GTK 4 Rust API](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/) — Rust bindings for GTK 4.
- [GTK 4 Visual Index](https://docs.gtk.org/gtk4/visual_index.html) — Visual reference for GTK widgets.
- [GTK Official Website](https://www.gtk.org/) — GTK project and general documentation.

### gtk-rs

- [GUI Development with Rust and GTK 4](https://gtk-rs.org/gtk4-rs/stable/latest/book/) — Official gtk-rs book.
- [gtk4-rs GitHub Repository](https://github.com/gtk-rs/gtk4-rs) — Source code and examples.
- [gtk-rs Project](https://gtk-rs.org/) — Rust bindings for GTK and related libraries.

### GLib

- [GLib Rust API](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/) — GLib bindings, macros, objects, signals and utilities.
- [GLib Official Documentation](https://docs.gtk.org/glib/) — GLib reference documentation.

### Related Libraries

- [GIO Rust API](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/) — Application infrastructure, file operations and asynchronous I/O.
- [GDK 4 Rust API](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gdk/) — Display, input and window-system integration.
- [Pango Rust API](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/pango/) — Text layout and typography.
- [Cairo Rust API](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/cairo/) — 2D graphics rendering.

---

## Development

Ducky is developed using Rust and GTK 4. Its source code and workspace files are managed using standard Rust and filesystem tools.

The official documentation above serves as the primary reference during development.