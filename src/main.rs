use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{self, Align, Application, ApplicationWindow, Overlay, glib};
use gtk::{CssProvider, prelude::*};

mod controls;
mod grid;

const APP_ID: &str = "org.nexus.Simulator";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
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

fn build_exit(app: &Application) -> gtk::Button {
    let exit_button = gtk::Button::builder()
        .label("x")
        .name("exit-button")
        .halign(Align::End)
        .valign(Align::Start)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    exit_button.connect_clicked(glib::clone!(
        #[weak]
        app,
        move |_| {
            app.quit();
        }
    ));
    exit_button.add_css_class("red");
    exit_button.add_css_class("bg-transparent");

    exit_button
}

fn build_ui(app: &Application) {
    let title = build_title();
    let exit = build_exit(app);
    let grid = grid::build_grid();
    let controls = controls::build_controls();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 15);
    vbox.append(&title);
    vbox.append(&grid);
    vbox.append(&controls);

    let overlay = Overlay::new();
    overlay.add_overlay(&vbox);
    overlay.add_overlay(&exit);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Nexus")
        .name("nexus-window")
        .default_width(800)
        .default_height(600)
        .child(&overlay)
        .build();

    window.present();
}
