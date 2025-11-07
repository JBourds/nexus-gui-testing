use std::cell::Cell;
use std::rc::Rc;

use glib::clone;
use gtk::gdk::Display;
use gtk::{
    self, Application, ApplicationWindow, Button, ColorButton, Orientation, gdk::RGBA, glib,
};
use gtk::{Align, CssProvider, prelude::*};

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

fn sim_controls() -> gtk::Box {
    let b_start = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .can_shrink(true)
        .can_focus(true)
        .valign(Align::Center)
        .build();
    b_start.add_css_class("bg-green");
    b_start.connect_clicked(|_| {
        println!("Started!");
    });

    let b_pause = Button::builder()
        .label("Pause")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .can_shrink(true)
        .can_focus(true)
        .valign(Align::Center)
        .build();
    b_pause.add_css_class("bg-yellow");
    b_pause.connect_clicked(|_| {
        println!("Paused!");
    });

    let b_stop = Button::builder()
        .label("Stop")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .can_shrink(true)
        .can_focus(true)
        .vexpand(false)
        .valign(Align::Center)
        .build();
    b_stop.add_css_class("bg-red");
    b_stop.connect_clicked(|_| {
        println!("Stopped!");
    });

    let controls = gtk::Box::builder()
        .halign(Align::Center)
        .orientation(Orientation::Horizontal)
        .build();
    controls.append(&b_start);
    controls.append(&b_pause);
    controls.append(&b_stop);
    controls
}

fn build_ui(app: &Application) {
    let controls = sim_controls();
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Nexus")
        .child(&controls)
        .build();

    window.present();
}
