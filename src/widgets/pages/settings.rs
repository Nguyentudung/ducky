use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::core::i18n::language::{AppLanguage, LanguageManager};

pub fn build(manager: Rc<RefCell<LanguageManager>>, on_language_changed: Rc<dyn Fn()>) -> gtk::Box {
    let settings = gtk::Box::new(gtk::Orientation::Vertical, 18);

    settings.set_margin_top(24);
    settings.set_margin_bottom(24);
    settings.set_margin_start(24);
    settings.set_margin_end(24);

    // Page title
    let title = gtk::Label::new(None);
    title.add_css_class("title-1");
    title.set_halign(gtk::Align::Start);

    // Language section title
    let language_title = gtk::Label::new(None);
    language_title.add_css_class("title-3");
    language_title.set_halign(gtk::Align::Start);

    // Description
    let description = gtk::Label::new(None);
    description.set_halign(gtk::Align::Start);
    description.set_wrap(true);
    description.add_css_class("dim-label");

    // Language row
    let language_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    language_row.set_hexpand(true);

    let language_label = gtk::Label::new(None);
    language_label.set_halign(gtk::Align::Start);
    language_label.set_hexpand(true);

    let language_model = gtk::StringList::new(&["System Default", "English", "Tiếng Việt"]);

    let language_dropdown = gtk::DropDown::new(Some(language_model), None::<gtk::Expression>);

    language_dropdown.set_width_request(180);

    language_row.append(&language_label);
    language_row.append(&language_dropdown);

    // Language section
    let language_section = gtk::Box::new(gtk::Orientation::Vertical, 8);

    language_section.append(&language_title);
    language_section.append(&description);
    language_section.append(&language_row);

    settings.append(&title);
    settings.append(&language_section);

    // Initialize selected language
    let selected = match manager.borrow().language() {
        AppLanguage::System => 0,
        AppLanguage::English => 1,
        AppLanguage::Vietnamese => 2,
    };

    language_dropdown.set_selected(selected);

    // Update Settings labels
    let update_labels: Rc<dyn Fn()> = {
        let manager = manager.clone();
        let title = title.clone();
        let language_title = language_title.clone();
        let description = description.clone();
        let language_label = language_label.clone();

        Rc::new(move || {
            let manager = manager.borrow();

            title.set_text(&manager.translate("settings-title"));

            language_title.set_text(&manager.translate("settings-language"));

            description.set_text(&manager.translate("settings-language-description"));

            language_label.set_text(&manager.translate("settings-language-label"));
        })
    };

    update_labels();

    // Language selection
    {
        let manager = manager.clone();
        let update_labels = update_labels.clone();
        let on_language_changed = on_language_changed.clone();

        language_dropdown.connect_selected_notify(move |dropdown| {
            let language = match dropdown.selected() {
                1 => AppLanguage::English,
                2 => AppLanguage::Vietnamese,
                _ => AppLanguage::System,
            };

            manager.borrow_mut().set_language(language);

            // Update Settings page
            update_labels();

            // Update other application widgets
            on_language_changed();
        });
    }

    settings
}
