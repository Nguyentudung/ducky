use adw::prelude::*;

use gtk::glib;

use crate::widgets::sidebar;

pub struct DuckyApplication {
    app: adw::Application,
}

impl DuckyApplication {
    pub fn new() -> Self {
        let app = adw::Application::builder()
            .application_id("io.github.nguyentudung.Ducky")
            .build();

        app.connect_activate(|app| {
            crate::ui::styles::load();

            // Main window
            let window = adw::ApplicationWindow::builder()
                .application(app)
                .title("Ducky")
                .default_width(1000)
                .default_height(700)
                .build();

            // Header Bar
            let header_bar = adw::HeaderBar::new();

            let title = gtk::Label::new(Some("Ducky"));

            header_bar.set_title_widget(Some(&title));

            // Sidebar
            let sidebar = sidebar::build();

            let sidebar_page = adw::NavigationPage::new(&sidebar, "Navigation");

            // Content Stack
            let content_stack = gtk::Stack::new();

            content_stack.set_transition_type(gtk::StackTransitionType::Crossfade);

            content_stack.set_transition_duration(200);

            content_stack.set_hexpand(true);
            content_stack.set_vexpand(true);

            // Dashboard
            let dashboard = gtk::Box::new(gtk::Orientation::Vertical, 12);

            dashboard.set_halign(gtk::Align::Center);
            dashboard.set_valign(gtk::Align::Center);

            let dashboard_title = gtk::Label::new(Some("Dashboard"));

            dashboard_title.add_css_class("title-1");

            // Create Problem button
            let create_button = gtk::Button::with_label("Create Problem");

            create_button.add_css_class("suggested-action");
            create_button.add_css_class("rounded-button");
            create_button.set_size_request(160, 50);

            // Delete Problem button
            let delete_button = gtk::Button::with_label("Delete Problem");

            delete_button.add_css_class("destructive-action");
            delete_button.add_css_class("rounded-button");
            delete_button.set_size_request(160, 50);

            dashboard.append(&dashboard_title);
            dashboard.append(&create_button);
            dashboard.append(&delete_button);

            // Problems page
            let problems = gtk::Box::new(gtk::Orientation::Vertical, 12);

            problems.set_margin_top(24);
            problems.set_margin_bottom(24);
            problems.set_margin_start(24);
            problems.set_margin_end(24);

            let problems_title = gtk::Label::new(Some("Problems"));

            problems_title.add_css_class("title-1");
            problems_title.set_halign(gtk::Align::Start);

            problems.append(&problems_title);

            // Recent page
            let recent = gtk::Box::new(gtk::Orientation::Vertical, 12);

            recent.set_margin_top(24);
            recent.set_margin_bottom(24);
            recent.set_margin_start(24);
            recent.set_margin_end(24);

            let recent_title = gtk::Label::new(Some("Recent"));

            recent_title.add_css_class("title-1");
            recent_title.set_halign(gtk::Align::Start);

            recent.append(&recent_title);

            // Settings page
            let settings = gtk::Box::new(gtk::Orientation::Vertical, 12);

            settings.set_margin_top(24);
            settings.set_margin_bottom(24);
            settings.set_margin_start(24);
            settings.set_margin_end(24);

            let settings_title = gtk::Label::new(Some("Settings"));

            settings_title.add_css_class("title-1");
            settings_title.set_halign(gtk::Align::Start);

            settings.append(&settings_title);

            // Add pages to Stack
            content_stack.add_named(&dashboard, Some("dashboard"));

            content_stack.add_named(&problems, Some("problems"));

            content_stack.add_named(&recent, Some("recent"));

            content_stack.add_named(&settings, Some("settings"));

            // Clone Stack for navigation callback
            let navigation_stack = content_stack.clone();

            // Navigation selection
            sidebar.connect_row_selected(move |_, row| {
                if let Some(row) = row {
                    match row.index() {
                        0 => {
                            navigation_stack.set_visible_child_name("dashboard");
                        }

                        1 => {
                            navigation_stack.set_visible_child_name("problems");
                        }

                        2 => {
                            navigation_stack.set_visible_child_name("recent");
                        }

                        3 => {
                            navigation_stack.set_visible_child_name("settings");
                        }

                        _ => {}
                    }
                }
            });

            // Select Dashboard initially
            sidebar.select_row(sidebar.row_at_index(0).as_ref());

            // Content Navigation Page
            let content_page = adw::NavigationPage::new(&content_stack, "Ducky");

            // Navigation Split View
            let split_view = adw::NavigationSplitView::new();

            split_view.set_sidebar(Some(&sidebar_page));

            split_view.set_content(Some(&content_page));

            split_view.set_sidebar_width_fraction(0.25);
            split_view.set_min_sidebar_width(180.0);
            split_view.set_max_sidebar_width(280.0);

            // Toolbar View
            let toolbar_view = adw::ToolbarView::new();

            toolbar_view.add_top_bar(&header_bar);

            toolbar_view.set_content(Some(&split_view));

            window.set_content(Some(&toolbar_view));

            window.present();
        });

        Self { app }
    }

    pub fn run(self) -> glib::ExitCode {
        self.app.run()
    }
}
