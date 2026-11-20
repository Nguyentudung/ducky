# Linux Development & Build Guide

This guide covers the development environment setup, building, running and troubleshooting Ducky on Linux.

The instructions are primarily intended for Ubuntu and Ubuntu-based distributions.

## Requirements

- Linux
- Git
- Rust toolchain
- GTK 4 development libraries
- Build tools

## 1. Install System Dependencies

On Ubuntu, install the required packages:

```bash
sudo apt update
sudo apt install build-essential pkg-config libgtk-4-dev git
```

| Package | Purpose |
|---|---|
| `build-essential` | Compiler and build tools |
| `pkg-config` | Detect system libraries and compiler flags |
| `libgtk-4-dev` | GTK 4 development libraries and headers |
| `git` | Source code version control |

## 2. Install Rust

Install Rust using rustup:

https://rustup.rs/

Verify the installation:

```bash
rustc --version
cargo --version
```

If Rust or Cargo cannot be found, reload the Cargo environment:

```bash
source "$HOME/.cargo/env"
```

## 3. Verify GTK 4

Check the GTK 4 version:

```bash
pkg-config --modversion gtk4
```

If a version number is returned, GTK 4 is available through `pkg-config`.

## 4. Clone the Repository

Clone Ducky:

```bash
git clone <repository-url>
```

Navigate to the project directory:

```bash
cd ducky
```

## 5. Run in Development

Check the project without producing an executable:

```bash
cargo check
```

Build the debug version:

```bash
cargo build
```

Run Ducky:

```bash
cargo run
```

## 6. Build

### Debug Build

```bash
cargo build
```

Executable:

```text
target/debug/ducky
```

Run the debug executable:

```bash
./target/debug/ducky
```

### Release Build

Build an optimized release version:

```bash
cargo build --release
```

Executable:

```text
target/release/ducky
```

Run the release executable:

```bash
./target/release/ducky
```

## 7. Useful Cargo Commands

| Command | Description |
|---|---|
| `cargo check` | Check code without producing an executable |
| `cargo build` | Build the debug version |
| `cargo build --release` | Build the optimized release version |
| `cargo run` | Build and run the debug version |
| `cargo run --release` | Build and run the release version |
| `cargo clean` | Remove build artifacts |
| `cargo update` | Update dependencies within version constraints |
| `cargo tree` | Display the dependency tree |
| `cargo fmt` | Format Rust source code |
| `cargo clippy` | Run Rust lint checks |

## 8. Build Output

```text
target/
├── debug/
│   └── ducky
└── release/
    └── ducky
```

## 9. Runtime Dependencies

The compiled application may require GTK 4 and other system libraries at runtime.

A release build does not automatically bundle all required runtime dependencies. The target Linux system must have the required libraries installed.

## Troubleshooting

### GTK 4 cannot be found

Install GTK 4 development libraries and `pkg-config`:

```bash
sudo apt install libgtk-4-dev pkg-config
```

Verify GTK:

```bash
pkg-config --modversion gtk4
```

### Rust or Cargo cannot be found

Reload the Cargo environment:

```bash
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version
cargo --version
```

### Build fails

Ensure system dependencies are installed and up to date:

```bash
sudo apt update
sudo apt install build-essential pkg-config libgtk-4-dev
```

Then check and rebuild:

```bash
cargo check
cargo build
```

### Remove Build Artifacts

Clean the project's build output:

```bash
cargo clean
```
