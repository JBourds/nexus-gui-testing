use core::fmt;
use gtk::Button;
use gtk::CssProvider;
use gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
use gtk::prelude::*;
use gtk::{Align, prelude::*};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gtk::{Label, ListBox, Orientation, PolicyType, ProgressBar, ScrolledWindow};

use crate::colors::battery_colors;

pub const NODES: [Node; 10] = [
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
    Node {
        name: "Node 6",
        coords: Coords {
            x: -30.0,
            y: -40.0,
            z: 0.0,
        },
        battery: 100.0,
    },
    Node {
        name: "Node 7",
        coords: Coords {
            x: -30.0,
            y: -40.0,
            z: 0.0,
        },
        battery: 100.0,
    },
    Node {
        name: "Node 8",
        coords: Coords {
            x: -30.0,
            y: -40.0,
            z: 0.0,
        },
        battery: 100.0,
    },
    Node {
        name: "Node 9",
        coords: Coords {
            x: -30.0,
            y: -40.0,
            z: 0.0,
        },
        battery: 100.0,
    },
    Node {
        name: "Node 10",
        coords: Coords {
            x: -30.0,
            y: -40.0,
            z: 0.0,
        },
        battery: 100.0,
    },
];

#[derive(Clone, Copy, Default, Debug)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Node {
    pub name: &'static str,
    pub coords: Coords,
    pub battery: f64,
}

fn build_node_widget(node: &Node) -> gtk::Box {
    let vbox = gtk::Box::new(Orientation::Vertical, 10);
    vbox.set_halign(Align::Fill);
    vbox.set_hexpand(true);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);

    let name_label = Label::new(Some(node.name));

    let coords_label = Label::new(Some(&format!("{}", node.coords)));

    let battery_bar = ProgressBar::new();
    battery_bar.set_halign(Align::Fill);
    battery_bar.set_fraction(node.battery / 100.0);
    battery_bar.set_show_text(false);
    battery_bar.set_margin_top(2);
    battery_bar.set_margin_bottom(2);
    battery_bar.set_hexpand(true);
    battery_bar.set_vexpand(false);

    let battery_box = gtk::Box::new(Orientation::Vertical, 5);
    battery_box.append(&battery_bar);
    let battery_label = Label::new(Some(&format!("{:.0}%", node.battery)));
    battery_box.append(&battery_label);

    vbox.append(&name_label);
    vbox.append(&coords_label);
    vbox.append(&battery_box);

    vbox
}

pub fn build_node_list(nodes: Rc<RefCell<HashMap<String, Node>>>) -> gtk::Box {
    // Create the ListBox and populate it
    let list_box = ListBox::builder()
        .valign(Align::Center)
        .vexpand(true)
        .build();

    // Lexicographical sort by node class
    let borrow = nodes.borrow();
    let mut names = borrow.keys().collect::<Vec<&String>>();
    names.sort();
    for key in names {
        list_box.append(&build_node_widget(borrow.get(key).unwrap()));
    }

    let minimize_button = Button::with_label("−");
    minimize_button.set_halign(Align::End);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .name("node-scroll")
        .min_content_width(400)
        .max_content_width(600)
        .child(&list_box)
        .build();

    let scrolled_window_clone = scrolled_window.clone();
    minimize_button.connect_clicked(move |btn| {
        let is_visible = scrolled_window_clone.is_visible();
        scrolled_window_clone.set_visible(!is_visible);

        if is_visible {
            btn.set_label("+");
        } else {
            btn.set_label("−");
        }
    });

    let vbox = gtk::Box::new(Orientation::Vertical, 10);
    vbox.set_halign(Align::End);
    vbox.append(&minimize_button);
    vbox.append(&scrolled_window);

    vbox
}
