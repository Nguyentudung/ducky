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
            let window = adw::ApplicationWindow::builder()
                .application(app)
                .title("Ducky")
                .default_width(800)
                .default_height(600)
                .build();

            // Header Bar
            let header_bar = adw::HeaderBar::new();

            let title = gtk::Label::new(Some("Ducky"));

            header_bar.set_title_widget(Some(&title));

            // Main content
            let content = gtk::Box::new(gtk::Orientation::Vertical, 0);

            // Toolbar view
            let toolbar_view = adw::ToolbarView::new();
            toolbar_view.add_top_bar(&header_bar);
            toolbar_view.set_content(Some(&content));

            window.set_content(Some(&toolbar_view));

            window.present();
        });

        Self { app }
    }

    pub fn run(self) -> glib::ExitCode {
        self.app.run()
    }
}
