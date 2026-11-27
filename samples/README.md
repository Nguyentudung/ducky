# Ducky Language Samples

This directory stores the built-in boilerplate templates used by Ducky when generating files for new problems and when adding language files to an existing problem.

Instead of hardcoding starter code inside Rust source files, Ducky keeps each language boilerplate as a real, editable file under `samples/`.

## Template Philosophy

Starter templates are intentionally **minimal, algorithm-neutral boilerplates**:

- Provide only the minimal language entry point and essential standard imports/I/O setup.
- **Do not** include pre-written algorithms, sorting/searching helpers, or problem solutions.
- Leave a clean placeholder (`// TODO: Write your solution here` or `# TODO: Write your solution here`) so the learner implements the logic themselves.
- `problem-readme.md` provides structured sections for title, statement, input/output, notes, and complexity analysis filled in by the user.

## Template Structure

```text
samples/
├── README.md            # Contributor documentation (this file)
├── problem-readme.md    # Default problem README.md template with {{placeholders}}
├── c/
│   └── main.c           # Minimal C entry point (<stdio.h>, <stdlib.h>, main)
├── cpp/
│   └── main.cpp         # Minimal C++ boilerplate (fast I/O, main)
├── python/
│   └── main.py          # Minimal Python boilerplate (solve + __main__)
├── rust/
│   └── main.rs          # Minimal Rust entry point (empty main)
└── java/
    └── Main.java        # Minimal Java Main class
```

## Supported Languages

| Language ID | Display Name | Sample File | Default Workspace Filename |
| --- | --- | --- | --- |
| `c` | C | `c/main.c` | `main.c` |
| `cpp` | C++ | `cpp/main.cpp` | `main.cpp` |
| `python` | Python | `python/main.py` | `solution.py` |
| `rust` | Rust | `rust/main.rs` | `main.rs` |
| `java` | Java | `java/Main.java` | `Main.java` |

## How Ducky Loads Sample Files

`ducky-core` resolves sample templates through `crates/ducky-core/src/samples.rs`:

1. **Runtime directory lookup (first priority):** Ducky searches for a `samples/` directory on disk in this order:
   - `$DUCKY_SAMPLES_DIR` environment variable (if set)
   - `./samples` (relative to the current working directory)
   - `../samples` or `../../samples`
   - `<CARGO_MANIFEST_DIR>/../../samples`
2. **Compile-time resource fallback:** If `ducky` is executed as a standalone binary without a `samples/` directory beside it, Ducky falls back to the contents bundled via `include_str!` directly from `samples/` at build time.
3. **Database synchronization:** On startup, `sync_builtin_templates` seeds or refreshes unmodified built-in templates in SQLite (`templates` table). Templates edited by the user in Settings are never overwritten.

## Modifying or Replacing an Existing Sample

1. Edit the corresponding file directly inside `samples/` (for example, `samples/cpp/main.cpp` or `samples/problem-readme.md`).
2. Keep the boilerplate minimal, self-contained, and free of pre-implemented algorithms.
3. Restart Ducky (or rebuild if packaging a standalone binary).
4. For `samples/problem-readme.md`, you may use placeholders: `{{title}}`, `{{identifier}}`, `{{platform}}`, `{{url}}`, `{{difficulty}}`, `{{status}}`, `{{tags}}`, `{{languages}}`, `{{description}}`, `{{notes}}`, `{{created_at}}`, `{{updated_at}}`.

## Adding a New Built-in Language

1. Create a folder `samples/<language_id>/` using a lowercase alphanumeric slug (`a-z`, `0-9`, `-`).
2. Add the minimal starter file inside `samples/<language_id>/` following the naming rules below.
3. Register the language in `crates/ducky-core/migrations/` (if adding to built-in seed data) and add its mapping entry to `BUILTIN_SAMPLES` in `crates/ducky-core/src/samples.rs`.
4. Run `cargo test` to verify database seeding and workspace file generation.

> Note: End users can also add custom languages and custom templates at any time in **Settings -> Languages** and **Settings -> Templates** without modifying source code.

## File Naming Rules

- Language directory names must be lowercase ASCII slugs matching the language ID (e.g., `c`, `cpp`, `python`, `rust`, `java`).
- Use standard entry-point filenames:
  - `main.<ext>` for C, C++, Rust, Python (`main.c`, `main.cpp`, `main.rs`, `main.py`).
  - `Main.java` for Java so `public class Main` matches the filename cleanly.
- Avoid spaces, special characters, or platform-specific paths in sample filenames.
