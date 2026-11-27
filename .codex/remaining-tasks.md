# Remaining Tasks & Known Limitations

## Unverified Environments (Outside Current Host)

- **Windows (`x86_64-pc-windows-msvc`)** and **macOS (`aarch64-apple-darwin`, `x86_64-apple-darwin`)**: Cross-platform abstractions (`directories`, `open`, `std::path`, `tokio::signal::ctrl_c`, reserved Windows filename checks) and GitHub Actions release matrix (`.github/workflows/release.yml`) are implemented, but native compilation and runtime execution on Windows and macOS have not been verified in this Linux environment.
- **Desktop GUI browser auto-launch & file manager integration**: `open::that_detached` is wired for launching the default browser and opening workspace/problem folders, and `--no-open` was tested in headless CI/QA; actual desktop shell launch behavior depends on the user's desktop environment (`xdg-open` on Linux).

## Future Optional Enhancements

- Native OS directory picker dialog if desired (currently users paste or type an absolute workspace path and validate it live via `/api/workspace/validate`).
- Pagination / virtualized rendering if a user's collection grows to thousands of problems.
- Optional export/import of problem metadata & templates as JSON.
