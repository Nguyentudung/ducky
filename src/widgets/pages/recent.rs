use gtk::prelude::*;

pub fn build() -> gtk::Box {
    let recent = gtk::Box::new(gtk::Orientation::Vertical, 12);

    recent.set_margin_top(24);
    recent.set_margin_bottom(24);
    recent.set_margin_start(24);
    recent.set_margin_end(24);

    let title = gtk::Label::new(Some("Recent"));
    title.add_css_class("title-1");
    title.set_halign(gtk::Align::Start);

    recent.append(&title);

    recent
}
