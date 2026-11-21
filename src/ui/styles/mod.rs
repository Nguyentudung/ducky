use std::fs;
use std::path::Path;

pub fn load() {
    let display = gtk::gdk::Display::default().expect("Failed to get GTK display");

    let styles_directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ui/styles");

    let entries = fs::read_dir(&styles_directory).expect("Failed to read styles directory");

    let mut stylesheets = Vec::new();

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|extension| extension == "css") {
            stylesheets.push(path);
        }
    }

    stylesheets.sort();

    for stylesheet in stylesheets {
        let css_provider = gtk::CssProvider::new();

        css_provider.load_from_file(&gtk::gio::File::for_path(&stylesheet));

        gtk::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
