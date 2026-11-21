use adw::prelude::*;
use gtk::glib;

pub struct DuckyApplication {
    app: adw::Application,
}

impl DuckyApplication {
    pub fn new() -> Self {
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
