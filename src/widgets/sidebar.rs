use gtk::prelude::*;

pub fn build() -> gtk::ListBox {
    let sidebar = gtk::ListBox::new();

    sidebar.set_selection_mode(gtk::SelectionMode::Single);
    sidebar.add_css_class("navigation-sidebar");

    // Home
    add_navigation_item(&sidebar, "Home", "go-home-symbolic", "home");

    // Problems
    add_navigation_item(&sidebar, "Problems", "view-list-symbolic", "problems");

    // Recent
    add_navigation_item(
        &sidebar,
        "Recent",
        "document-open-recent-symbolic",
        "recent",
    );

    // Settings
    add_navigation_item(
        &sidebar,
        "Settings",
        "preferences-system-symbolic",
        "settings",
    );

    sidebar
}

fn add_navigation_item(sidebar: &gtk::ListBox, title: &str, icon_name: &str, page_name: &str) {
    let row = gtk::ListBoxRow::new();

    // Row content
    let content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    content.set_margin_start(12);
    content.set_margin_end(12);
    content.set_margin_top(8);
    content.set_margin_bottom(8);

    // Icon
    let icon = gtk::Image::from_icon_name(icon_name);

    icon.set_pixel_size(20);

    // Label
    let label = gtk::Label::new(Some(title));

    label.set_halign(gtk::Align::Start);
    label.set_hexpand(true);

    // Assemble row
    content.append(&icon);
    content.append(&label);

    row.set_child(Some(&content));

    // Page name for navigation
    row.set_widget_name(page_name);

    sidebar.append(&row);
}
