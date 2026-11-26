//! Native Slint application entry point. Domain services intentionally do not depend on Slint.

pub mod app;
pub mod core;
pub mod platform;
pub mod state;

slint::include_modules!();

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
