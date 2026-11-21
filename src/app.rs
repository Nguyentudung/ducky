use gtk4::prelude::*;

pub fn on_activate(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("Ducky"));
    window.set_default_size(800, 600);

    window.present();
}
