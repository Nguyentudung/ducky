# Workspace Behavior & File Generation

Ducky keeps every programming exercise as normal files on disk so your workspace works seamlessly with any editor, compiler, or Git repository.

## Directory Layout

When creating a problem, Ducky places it inside your configured workspace root:

- **Platform grouping (`group_by_platform = true`, default)**:
  - If `platform` is provided (e.g. `Codeforces`), Ducky creates `<workspace>/<Platform>/<ProblemFolder>/`.
  - If `platform` is empty, Ducky creates `<workspace>/<ProblemFolder>/`.
- **Platform grouping (`group_by_platform = false`)**:
  - Always creates `<workspace>/<ProblemFolder>/`.

## File Layout Modes

- **`auto` (default)**: Places a single language file directly in the problem folder (`main.cpp`), and uses language subdirectories (`cpp/main.cpp`, `rust/main.rs`) when multiple languages are selected.
- **`subdirs`**: Always places source files under `<lang_folder>/<filename>`.
- **`flat`**: Always places source files directly in `<ProblemFolder>/<filename>` (rejecting duplicate filenames before touching disk).

## Safety Guarantees

1. **Atomic folder creation**: Files are written to a temporary sibling staging directory and renamed into place only when all files succeed.
2. **No silent overwrites**: Source files use `create_new(true)`. Regenerating an existing `README.md` requires explicit confirmation (`overwrite: true`).
3. **Non-destructive problem deletion**: Deleting a problem removes only its SQLite metadata by default; deleting the folder on disk is opt-in and restricted to directories strictly inside the configured workspace root.
4. **Path safety**: Relative paths, `..` traversal, disallowed characters, Windows reserved names, and symlink escapes outside the workspace root are rejected.
