pub mod application;
pub mod config;
pub mod core;
pub mod ui;
pub mod utils;
pub mod widgets;

mod deps {
    pub use gtk4::{gio, glib};
}

use application::DuckyApplication;
use deps::*;

pub fn main() -> glib::ExitCode {
    DuckyApplication::new().run()
}
