use gtk::gdk::Display;
use gtk::{self, Application, ApplicationWindow, glib};
use gtk::{CssProvider, prelude::*};

mod controls;
mod grid;

const APP_ID: &str = "org.nexus.Simulator";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_title() -> gtk::Label {
    gtk::Label::builder()
        .label("Nexus")
        .margin_start(24)
        .margin_end(24)
        .margin_top(12)
        .margin_bottom(12)
        .name("title")
        .build()
}

fn build_ui(app: &Application) {
    let title = build_title();
    let grid = grid::build_grid();
    let controls = controls::build_controls();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 15);
    vbox.append(&title);
    vbox.append(&grid);
    vbox.append(&controls);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Nexus")
        .default_width(800)
        .default_height(600)
        .child(&vbox)
        .build();

    window.present();
}
