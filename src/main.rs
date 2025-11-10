use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::{self, Align, Application, ApplicationWindow, Overlay, glib};
use gtk::{Button, prelude::*};
use gtk::{CssProvider, prelude::*};

use crate::grid::imp::{Coords, Node};

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
    let nodes = Rc::new(RefCell::new(
        [
            Node {
                name: "Node 1",
                coords: Coords {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                battery: 100.0,
            },
            Node {
                name: "Node 2",
                coords: Coords {
                    x: 100.0,
                    y: 0.0,
                    z: 0.0,
                },
                battery: 75.0,
            },
            Node {
                name: "Node 3",
                coords: Coords {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                battery: 0.0,
            },
            Node {
                name: "Node 4",
                coords: Coords {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                battery: 0.01,
            },
            Node {
                name: "Node 5",
                coords: Coords {
                    x: -30.0,
                    y: -40.0,
                    z: 0.0,
                },
                battery: 100.0,
            },
        ]
        .into_iter()
        .fold(HashMap::new(), |mut map, n| {
            map.insert(n.name.to_string(), n);
            map
        }),
    ));
    let title = build_title();
    let exit = build_exit(app);
    let grid = grid::build_grid(nodes);
    let controls = controls::build_controls();
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(clone!(
        #[weak]
        grid,
        move |_| {
            grid.update_view();
        }
    ));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 15);
    vbox.append(&title);
    vbox.append(&grid);
    vbox.append(&controls);
    vbox.append(&button);

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
