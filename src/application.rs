use gtk4::{Application, ApplicationWindow, gio, glib, prelude::*};

pub struct DuckyApplication {
    app: Application,
}

impl DuckyApplication {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("io.github.nguyentudung.Ducky")
            .build();

        app.connect_activate(|app| {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("Ducky")
                .default_width(800)
                .default_height(600)
                .build();

            window.present();
        });

        Self { app }
    }

    pub fn run(self) -> glib::ExitCode {
        self.app.run()
    }
}
