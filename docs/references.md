# References

Primary sources used for the Slint migration:

- [Slint 1.19.1 release](https://github.com/slint-ui/slint/releases/tag/v1.19.1) — selected stable version and patch-release status.
- [Slint Rust API](https://docs.slint.dev/latest/docs/rust/slint/) — external `.slint` compilation through `build.rs`, exported component handles, callbacks and UI-thread rules.
- [Slint functions and callbacks](https://docs.slint.dev/latest/docs/slint/guide/language/coding/functions-and-callbacks/) — UI-to-Rust callback boundary.
- [Slint models](https://docs.slint.dev/latest/docs/slint/tutorial/creating_the_tiles/) — `VecModel`/`ModelRc` model updates.
- [Slint Winit backend](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/) — Linux/Windows/macOS windowing and renderer selection.
- [Slint renderer feature documentation](https://docs.slint.dev/latest/docs/rust/slint/docs/cargo_features/) — `winit-software` fallback used during Linux launch verification.
- [Rust standard library](https://doc.rust-lang.org/std/) — safe file and path APIs.
- [rusqlite](https://docs.rs/rusqlite/) — SQLite persistence.
- [Project Fluent](https://projectfluent.org/) — English/Vietnamese localization resources.
