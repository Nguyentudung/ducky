use gtk::prelude::*;

pub fn build<F>(on_settings: F) -> (gtk::Box, gtk::ListBox)
where
    F: Fn() + 'static,
{
    let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Navigation
    let navigation = gtk::ListBox::new();

    navigation.set_selection_mode(gtk::SelectionMode::Single);
    navigation.add_css_class("navigation-sidebar");
    navigation.set_vexpand(true);

    add_navigation_item(&navigation, "Home", "go-home-symbolic", "home");
    add_navigation_item(&navigation, "Problems", "view-list-symbolic", "problems");
    add_navigation_item(
        &navigation,
        "Recent",
        "document-open-recent-symbolic",
        "recent",
    );

    // Bottom section
    let bottom_section = gtk::Box::new(gtk::Orientation::Vertical, 6);

    bottom_section.set_margin_start(8);
    bottom_section.set_margin_end(8);
    bottom_section.set_margin_bottom(8);

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

    // Settings Button
    let settings_button = gtk::Button::new();
    settings_button.add_css_class("flat");

    let settings_content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    settings_content.set_margin_start(12);
    settings_content.set_margin_end(12);
    settings_content.set_margin_top(8);
    settings_content.set_margin_bottom(8);

    let settings_icon = gtk::Image::from_icon_name("preferences-system-symbolic");
    settings_icon.set_pixel_size(20);

    let settings_label = gtk::Label::new(Some("Settings"));
    settings_label.set_halign(gtk::Align::Start);
    settings_label.set_hexpand(true);

    settings_content.append(&settings_icon);
    settings_content.append(&settings_label);

    settings_button.set_child(Some(&settings_content));

    settings_button.connect_clicked(move |_| {
        on_settings();
    });

    bottom_section.append(&separator);
    bottom_section.append(&settings_button);

    sidebar.append(&navigation);
    sidebar.append(&bottom_section);
    (sidebar, navigation)
}

fn add_navigation_item(sidebar: &gtk::ListBox, title: &str, icon_name: &str, page_name: &str) {
    let row = gtk::ListBoxRow::new();

    let content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    content.set_margin_start(12);
    content.set_margin_end(12);
    content.set_margin_top(8);
    content.set_margin_bottom(8);

    let icon = gtk::Image::from_icon_name(icon_name);
    icon.set_pixel_size(20);

    let label = gtk::Label::new(Some(title));
    label.set_halign(gtk::Align::Start);
    label.set_hexpand(true);

    content.append(&icon);
    content.append(&label);

    row.set_child(Some(&content));
    row.set_widget_name(page_name);

    sidebar.append(&row);
}
