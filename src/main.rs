use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
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
const NODES: [Node; 5] = [
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
];

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

#[derive(Debug)]
enum State {
    Paused,
    Running,
    Reset,
}

fn build_ui(app: &Application) {
    let nodes = Rc::new(RefCell::new(NODES.into_iter().fold(
        HashMap::new(),
        |mut map, n| {
            map.insert(n.name.to_string(), n);
            map
        },
    )));

    let title = build_title();
    let exit = build_exit(app);
    let grid = grid::build_grid(Rc::clone(&nodes));
    let state = Arc::new(Mutex::new(State::Paused));
    let controls = controls::build_controls(state.clone());

    let mut rand = rand::rng();
    let grid_clone = grid.clone();
    glib::timeout_add_local(Duration::from_millis(100), move || {
        let mut guard = state.lock().unwrap();
        match *guard {
            State::Paused => {}
            State::Running => {
                for (_, node) in nodes.borrow_mut().iter_mut() {
                    node.battery = (node.battery + (rand.random::<f64>() - 0.5) * 3.0).max(0.0);
                    if node.battery > 0.0 {
                        node.coords.x += rand.random::<f64>() * 5.0;
                        node.coords.y += rand.random::<f64>() * 5.0;
                        node.coords.z += rand.random::<f64>() * 5.0;
                    }
                }
                grid_clone.update_view();
            }
            State::Reset => {
                *nodes.borrow_mut() = NODES.into_iter().fold(HashMap::new(), |mut map, n| {
                    map.insert(n.name.to_string(), n);
                    map
                });
                *guard = State::Paused;
            }
        }
        grid_clone.update_view();
        glib::ControlFlow::Continue
    });
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
