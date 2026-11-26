//! Small cross-platform adapters. `open` chooses the platform-appropriate shell integration.

use std::path::Path;

pub fn open_path(path: &Path) -> Result<(), String> {
    open::that(path).map_err(|e| format!("Could not open {}: {e}", path.display()))
}
pub fn open_url(url: &str) -> Result<(), String> {
    open::that(url).map_err(|e| format!("Could not open URL: {e}"))
}
