use gtk::cairo;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{self, Align, Box as GtkBox, DrawingArea, Orientation, glib};
use imp::Coords;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::grid::imp::Node;

pub fn build_grid() -> NodeGrid {
    let grid = NodeGrid::new();
    grid.set_vexpand(true);
    grid.set_hexpand(true);
    grid.set_margin_top(24);
    grid.set_margin_bottom(24);
    grid.set_margin_start(24);
    grid.set_margin_end(24);
    grid
}

const NODES: [Node; 4] = [
    Node {
        name: "Node 1",
        coords: Coords {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
    },
    Node {
        name: "Node 2",
        coords: Coords {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    },
    Node {
        name: "Node 3",
        coords: Coords {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    },
    Node {
        name: "Node 4",
        coords: Coords {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    },
];

mod imp {
    use super::*;

    use gtk::glib::subclass::object::ObjectImpl;
    use gtk::glib::subclass::types::{ObjectSubclass, ObjectSubclassExt};
    use gtk::subclass::prelude::*;
    use gtk::subclass::widget::WidgetImpl;

    #[derive(Clone, Copy, Default, Debug)]
    pub struct Coords {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Default, Debug)]
    pub struct Node {
        pub name: &'static str,
        pub coords: Coords,
    }

    // NodeGrid will manage the state and drawing logic
    #[derive(Default)]
    pub struct NodeGrid {
        pub nodes: RefCell<HashMap<String, Node>>,
    }

    impl NodeGrid {
        fn update_view(&self) {
            self.obj().queue_draw();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NodeGrid {
        const NAME: &'static str = "NexusNodeGrid";
        type Type = super::NodeGrid;
        type ParentType = gtk::DrawingArea;
    }

    impl ObjectImpl for NodeGrid {
        fn constructed(&self) {
            self.parent_constructed();
            let widget = self.obj();
            widget.set_draw_func(|area, cr, width, height| {
                draw_cartesian_grid(cr, width as f64, height as f64);
                draw_points(cr);
            });
        }
    }
    impl WidgetImpl for NodeGrid {}
    impl DrawingAreaImpl for NodeGrid {}

    fn draw_cartesian_grid(cr: &cairo::Context, width: f64, height: f64) {
        cr.set_source_rgb(0.8, 0.8, 0.8);
        cr.set_line_width(1.0);

        for i in 0..=(width / 20.0) as i32 {
            let x = i as f64 * 20.0;
            cr.move_to(x, 0.0);
            cr.line_to(x, height);
            cr.stroke().expect("Failed to draw line");
        }

        for i in 0..=(height / 20.0) as i32 {
            let y = i as f64 * 20.0;
            cr.move_to(0.0, y);
            cr.line_to(width, y);
            cr.stroke().expect("Failed to draw line");
        }
    }

    fn draw_points(cr: &cairo::Context) {
        cr.set_source_rgb(1.0, 0.0, 0.0);
        cr.arc(100.0, 100.0, 5.0, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().expect("Failed to fill circle");

        cr.set_source_rgb(0.0, 0.0, 1.0);
        cr.arc(250.0, 300.0, 5.0, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().expect("Failed to fill circle");
    }
}

glib::wrapper! {
    pub struct NodeGrid(ObjectSubclass<imp::NodeGrid>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NodeGrid {
    fn new() -> Self {
        let obj: Self = glib::Object::new();
        {
            let imp = obj.imp();
            let mut nodes = imp.nodes.borrow_mut();
            for n in NODES.iter() {
                nodes.insert(n.name.to_string(), n.clone());
            }
        }
        obj
    }
}
