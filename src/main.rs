use gtk4::glib;
use gtk4::prelude::*;

mod app;

fn main() {
    glib::set_application_name("Ducky");

    let app = gtk4::Application::builder()
        .application_id("io.github.nguyentudung.ducky")
        .build();

    app.connect_activate(app::on_activate);

    app.run();
}
