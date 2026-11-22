use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::core::i18n::language::LanguageManager;

pub fn build<F>(
    manager: Rc<RefCell<LanguageManager>>,
    on_settings: F,
) -> (gtk::Box, gtk::ListBox, Rc<dyn Fn()>)
where
    F: Fn() + 'static,
{
    let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Navigation list
    let navigation = gtk::ListBox::new();
    navigation.set_selection_mode(gtk::SelectionMode::Single);
    navigation.add_css_class("navigation-sidebar");
    navigation.set_vexpand(true);

    // Navigation labels
    let home_label = gtk::Label::new(None);
    home_label.set_halign(gtk::Align::Start);
    home_label.set_hexpand(true);

    let problems_label = gtk::Label::new(None);
    problems_label.set_halign(gtk::Align::Start);
    problems_label.set_hexpand(true);

    let recent_label = gtk::Label::new(None);
    recent_label.set_halign(gtk::Align::Start);
    recent_label.set_hexpand(true);

    add_navigation_item(&navigation, &home_label, "go-home-symbolic");

    add_navigation_item(&navigation, &problems_label, "view-list-symbolic");

    add_navigation_item(&navigation, &recent_label, "document-open-recent-symbolic");

    // Bottom section
    let bottom_section = gtk::Box::new(gtk::Orientation::Vertical, 6);

    bottom_section.set_margin_start(8);
    bottom_section.set_margin_end(8);
    bottom_section.set_margin_bottom(8);

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

    // Settings button
    let settings_button = gtk::Button::new();
    settings_button.add_css_class("flat");

    let settings_content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    settings_content.set_margin_start(12);
    settings_content.set_margin_end(12);
    settings_content.set_margin_top(8);
    settings_content.set_margin_bottom(8);

    let settings_icon = gtk::Image::from_icon_name("preferences-system-symbolic");

    settings_icon.set_pixel_size(20);

    let settings_label = gtk::Label::new(None);
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

    // Update Sidebar labels
    let update_labels: Rc<dyn Fn()> = {
        let manager = manager.clone();

        let home_label = home_label.clone();
        let problems_label = problems_label.clone();
        let recent_label = recent_label.clone();
        let settings_label = settings_label.clone();

        Rc::new(move || {
            let manager = manager.borrow();

            home_label.set_text(&manager.translate("navigation-home"));

            problems_label.set_text(&manager.translate("navigation-problems"));

            recent_label.set_text(&manager.translate("navigation-recent"));

            settings_label.set_text(&manager.translate("settings-title"));
        })
    };

    // Initial labels
    update_labels();

    (sidebar, navigation, update_labels)
}

fn add_navigation_item(sidebar: &gtk::ListBox, label: &gtk::Label, icon_name: &str) {
    let row = gtk::ListBoxRow::new();

    let content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    content.set_margin_start(12);
    content.set_margin_end(12);
    content.set_margin_top(8);
    content.set_margin_bottom(8);

    let icon = gtk::Image::from_icon_name(icon_name);
    icon.set_pixel_size(20);

    content.append(&icon);
    content.append(label);

    row.set_child(Some(&content));
    sidebar.append(&row);
}
