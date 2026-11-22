use adw::prelude::*;
use gtk::glib;

use crate::core::database::{connection::open_database, schema::initialize_schema};

pub struct DuckyApplication {
    app: adw::Application,
}

impl DuckyApplication {
    pub fn new() -> Self {
        let conn = open_database().expect("Failed to open database");

        initialize_schema(&conn).expect("Failed to initialize database");

        let app = adw::Application::builder()
            .application_id("io.github.nguyentudung.Ducky")
            .build();

        app.connect_activate(|app| {
            crate::ui::styles::load();

            crate::widgets::window::build(app);
        });

        Self { app }
    }

    pub fn run(self) -> glib::ExitCode {
        self.app.run()
    }
}
