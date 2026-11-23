use gtk::prelude::*;

pub fn build() -> gtk::Box {
    let home = gtk::Box::new(gtk::Orientation::Vertical, 12);

    home.set_halign(gtk::Align::Center);
    home.set_valign(gtk::Align::Center);

    let title = gtk::Label::new(Some("Home"));
    title.add_css_class("title-1");

    home.append(&title);

    home
}
