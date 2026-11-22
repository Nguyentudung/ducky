use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::core::i18n::language::LanguageManager;

pub mod home;
pub mod problems;
pub mod recent;
pub mod settings;

pub fn build(
    manager: Rc<RefCell<LanguageManager>>,
    on_language_changed: Rc<dyn Fn()>,
) -> gtk::Stack {
    let stack = gtk::Stack::new();

    stack.set_transition_type(gtk::StackTransitionType::Crossfade);
    stack.set_transition_duration(200);
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    stack.add_named(&home::build(), Some("home"));
    stack.add_named(&problems::build(), Some("problems"));
    stack.add_named(&recent::build(), Some("recent"));

    stack.add_named(
        &settings::build(manager, on_language_changed),
        Some("settings"),
    );

    stack
}
