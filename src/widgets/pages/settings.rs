use gtk::prelude::*;

pub fn build() -> gtk::Box {
    let settings = gtk::Box::new(gtk::Orientation::Vertical, 12);

    settings.set_margin_top(24);
    settings.set_margin_bottom(24);
    settings.set_margin_start(24);
    settings.set_margin_end(24);

    let title = gtk::Label::new(Some("Settings"));
    title.add_css_class("title-1");
    title.set_halign(gtk::Align::Start);

    settings.append(&title);

    settings
}
