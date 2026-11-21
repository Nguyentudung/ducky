use gtk::prelude::*;

pub fn build() -> gtk::Box {
    let problems = gtk::Box::new(gtk::Orientation::Vertical, 12);

    problems.set_margin_top(24);
    problems.set_margin_bottom(24);
    problems.set_margin_start(24);
    problems.set_margin_end(24);

    let title = gtk::Label::new(Some("Problems"));
    title.add_css_class("title-1");
    title.set_halign(gtk::Align::Start);

    problems.append(&title);

    problems
}
