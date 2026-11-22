use adw::prelude::*;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::core::i18n::language::{AppLanguage, LanguageManager};
use crate::widgets::pages;
use crate::widgets::sidebar;

pub fn build(app: &adw::Application) {
    // Main Window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Ducky")
        .default_width(800)
        .default_height(500)
        .build();

    // Language Manager
    let language_manager = Rc::new(RefCell::new(LanguageManager::new(AppLanguage::System)));

    // Header Bars
    let sidebar_header = adw::HeaderBar::new();
    sidebar_header.add_css_class("sidebar-header");

    let content_header = adw::HeaderBar::new();
    content_header.add_css_class("content-header");

    let content_title = gtk::Label::new(None);
    content_title.set_halign(gtk::Align::Start);

    content_header.set_title_widget(Some(&content_title));

    // Current page tracking
    // 0 = Home, 1 = Problems, 2 = Recent, 3 = Settings
    let current_page = Rc::new(Cell::new(0usize));

    // Stack reference for Sidebar Settings button
    let stack_reference: Rc<RefCell<Option<gtk::Stack>>> = Rc::new(RefCell::new(None));

    // Sidebar
    let settings_stack_reference = stack_reference.clone();
    let settings_title = content_title.clone();
    let settings_manager = language_manager.clone();
    let settings_current_page = current_page.clone();

    let (sidebar_widget, sidebar_list, update_sidebar_labels) =
        sidebar::build(language_manager.clone(), move || {
            if let Some(stack) = settings_stack_reference.borrow().as_ref() {
                stack.set_visible_child_name("settings");
            }

            settings_current_page.set(3);

            settings_title.set_text(&settings_manager.borrow().translate("settings-title"));
        });

    // Update header title based on current page and language
    let update_header_title: Rc<dyn Fn()> = {
        let manager = language_manager.clone();
        let title = content_title.clone();
        let page = current_page.clone();

        Rc::new(move || {
            let manager = manager.borrow();

            let key = match page.get() {
                0 => "navigation-home",
                1 => "navigation-problems",
                2 => "navigation-recent",
                3 => "settings-title",
                _ => "navigation-home",
            };

            title.set_text(&manager.translate(key));
        })
    };

    // Global language change callback
    let update_language_ui: Rc<dyn Fn()> = {
        let update_sidebar = update_sidebar_labels.clone();
        let update_header = update_header_title.clone();

        Rc::new(move || {
            update_sidebar();
            update_header();
        })
    };

    // Pages Stack
    let content_stack = pages::build(language_manager.clone(), update_language_ui.clone());

    // Store stack for Settings button
    *stack_reference.borrow_mut() = Some(content_stack.clone());

    // Navigation
    let navigation_stack = content_stack.clone();
    let navigation_title = content_title.clone();
    let navigation_manager = language_manager.clone();
    let navigation_current_page = current_page.clone();

    sidebar_list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let index = row.index();

            let (page_name, translation_key) = match index {
                0 => ("home", "navigation-home"),
                1 => ("problems", "navigation-problems"),
                2 => ("recent", "navigation-recent"),
                _ => return,
            };

            navigation_stack.set_visible_child_name(page_name);
            navigation_current_page.set(index as usize);

            navigation_title.set_text(&navigation_manager.borrow().translate(translation_key));
        }
    });

    // Select Home Initially
    sidebar_list.select_row(sidebar_list.row_at_index(0).as_ref());

    // Sidebar Toolbar View
    let sidebar_toolbar = adw::ToolbarView::new();
    sidebar_toolbar.add_top_bar(&sidebar_header);
    sidebar_toolbar.set_content(Some(&sidebar_widget));

    // Sidebar Navigation Page
    let sidebar_page = adw::NavigationPage::new(&sidebar_toolbar, "Ducky");

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

    // Window Content
    window.set_content(Some(&split_view));

    // Present Window
    window.present();
}
