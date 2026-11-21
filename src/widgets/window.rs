use adw::prelude::*;

use crate::widgets::pages;
use crate::widgets::sidebar;

pub fn build(app: &adw::Application) {
    // Main Window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Ducky")
        .default_width(1000)
        .default_height(700)
        .build();

    // Sidebar Header Bar
    let sidebar_header = adw::HeaderBar::new();
    sidebar_header.add_css_class("sidebar-header");

    // Content Header Bar
    let content_header = adw::HeaderBar::new();
    content_header.add_css_class("content-header");

    let content_title = gtk::Label::new(Some("Home"));
    content_title.set_halign(gtk::Align::Start);

    content_header.set_title_widget(Some(&content_title));

    // Sidebar
    let sidebar = sidebar::build();

    // Sidebar Toolbar View
    let sidebar_toolbar = adw::ToolbarView::new();
    sidebar_toolbar.add_top_bar(&sidebar_header);
    sidebar_toolbar.set_content(Some(&sidebar));

    // Sidebar Navigation Page
    let sidebar_page = adw::NavigationPage::new(&sidebar_toolbar, "Ducky");

    // Pages Stack
    let content_stack = pages::build();

    // Navigation
    let navigation_stack = content_stack.clone();
    let navigation_title = content_title.clone();

    sidebar.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            match row.index() {
                0 => {
                    navigation_stack.set_visible_child_name("home");
                    navigation_title.set_text("Home");
                }

                1 => {
                    navigation_stack.set_visible_child_name("problems");
                    navigation_title.set_text("Problems");
                }

                2 => {
                    navigation_stack.set_visible_child_name("recent");
                    navigation_title.set_text("Recent");
                }

                3 => {
                    navigation_stack.set_visible_child_name("settings");
                    navigation_title.set_text("Settings");
                }

                _ => {}
            }
        }
    });

    // Select Home Initially
    sidebar.select_row(sidebar.row_at_index(0).as_ref());

    // Content Toolbar View
    let content_toolbar = adw::ToolbarView::new();
    content_toolbar.add_top_bar(&content_header);
    content_toolbar.set_content(Some(&content_stack));

    // Content Navigation Page
    let content_page = adw::NavigationPage::new(&content_toolbar, "Content");

    // Navigation Split View
    let split_view = adw::NavigationSplitView::new();

    split_view.set_sidebar(Some(&sidebar_page));
    split_view.set_content(Some(&content_page));

    split_view.set_sidebar_width_fraction(0.25);
    split_view.set_min_sidebar_width(180.0);
    split_view.set_max_sidebar_width(280.0);

    // Set Split View as Window Content
    window.set_content(Some(&split_view));

    // Present Window
    window.present();
}
