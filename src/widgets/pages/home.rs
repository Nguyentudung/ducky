use gtk::prelude::*;

pub fn build() -> gtk::Box {
    let home = gtk::Box::new(gtk::Orientation::Vertical, 12);

    home.set_halign(gtk::Align::Center);
    home.set_valign(gtk::Align::Center);

    let title = gtk::Label::new(Some("Home"));
    title.add_css_class("title-1");

    let create_button = gtk::Button::with_label("Create Problem");
    create_button.add_css_class("suggested-action");
    create_button.add_css_class("rounded-button");
    create_button.set_size_request(160, 50);

    let delete_button = gtk::Button::with_label("Delete Problem");
    delete_button.add_css_class("destructive-action");
    delete_button.add_css_class("rounded-button");
    delete_button.set_size_request(160, 50);

    home.append(&title);
    home.append(&create_button);
    home.append(&delete_button);

    home
}
