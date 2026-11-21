pub mod home;
pub mod problems;
pub mod recent;
pub mod settings;

use gtk::prelude::*;

pub fn build() -> gtk::Stack {
    let stack = gtk::Stack::new();

    stack.set_transition_type(gtk::StackTransitionType::Crossfade);
    stack.set_transition_duration(200);
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    stack.add_named(&home::build(), Some("home"));
    stack.add_named(&problems::build(), Some("problems"));
    stack.add_named(&recent::build(), Some("recent"));
    stack.add_named(&settings::build(), Some("settings"));

    stack
}
