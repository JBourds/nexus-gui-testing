use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gtk::CssProvider;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{self, Align, Application, Overlay, glib};

use crate::node::{NODES, build_node_list};

mod colors;
mod controls;
mod grid;
mod node;

const APP_ID: &str = "org.nexus.Simulator";

#[derive(Debug)]
enum State {
    Paused,
    Running,
    Reset,
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../style.css"));
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
        .halign(Align::Start)
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
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 15);
    vbox.append(&title);
    vbox.append(&grid);
    vbox.append(&controls);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    hbox.append(&vbox);

    let node_list = build_node_list(Rc::clone(&nodes));

    let overlay = Overlay::new();
    overlay.add_overlay(&hbox);
    overlay.add_overlay(&exit);
    overlay.add_overlay(&node_list);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Nexus")
        .name("nexus-window")
        .default_width(800)
        .default_height(600)
        .child(&overlay)
        .build();

    window.present();

    // Background task
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
}
