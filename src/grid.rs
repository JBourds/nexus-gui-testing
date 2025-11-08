use gtk::cairo;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{self, Align, Box as GtkBox, DrawingArea, Orientation, glib};
use imp::Coords;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::grid::imp::Node;

const NUM_SQUARES: usize = 10;
const NODE_RADIUS: f64 = 5.0;
const NODE_DIAMETER: f64 = 2.0 * NODE_RADIUS;
const GRID_MARGIN: f64 = 45.0;
const TEXT_PADDING: f64 = 5.0;

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
        pub battery: f64,
    }
    // NodeGrid will manage the state and drawing logic
    #[derive(Default)]
    pub struct NodeGrid {
        pub nodes: Rc<RefCell<HashMap<String, Node>>>,
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
            let nodes = self.nodes.clone();
            widget.set_draw_func(move |area, cr, width, height| {
                if nodes.borrow().is_empty() {
                    return;
                }
                let nodes = &nodes.borrow();
                let ctx = ScaleCtx::new(width as f64, height as f64, nodes);
                draw_cartesian_grid(cr, &ctx);
                draw_nodes(cr, &ctx, nodes);
            });
        }
    }
    impl WidgetImpl for NodeGrid {}
    impl DrawingAreaImpl for NodeGrid {}

    fn get_bounds(nodes: &HashMap<String, Node>) -> (f64, f64, f64, f64) {
        nodes.values().fold(
            (f64::MAX, f64::MIN, f64::MAX, f64::MIN),
            |(min_x, max_x, min_y, max_y), node| {
                (
                    min_x.min(node.coords.x),
                    max_x.max(node.coords.x),
                    min_y.min(node.coords.y),
                    max_y.max(node.coords.y),
                )
            },
        )
    }

    fn pixel_scales(width: f64, height: f64, range_x: f64, range_y: f64) -> (f64, f64) {
        let scale_x = (width - NODE_DIAMETER) / range_x;
        let scale_y = (height - NODE_DIAMETER) / range_y;
        (scale_x, scale_y)
    }

    #[derive(Debug)]
    struct ScaleCtx {
        pub min_x: f64,
        pub min_y: f64,
        pub max_x: f64,
        pub max_y: f64,
        pub scale_x: f64,
        pub scale_y: f64,
        pub range_x: f64,
        pub range_y: f64,
        pub width: f64,
        pub height: f64,
    }

    impl ScaleCtx {
        pub fn new(width: f64, height: f64, nodes: &HashMap<String, Node>) -> Self {
            let (min_x, max_x, min_y, max_y) = get_bounds(nodes);
            let range_x = max_x - min_x;
            let range_y = max_y - min_y;

            // The drawable width/height excludes the margins
            let drawable_width = width - 2.0 * GRID_MARGIN;
            let drawable_height = height - 2.0 * GRID_MARGIN;

            let scale_x = drawable_width / range_x;
            let scale_y = drawable_height / range_y;

            Self {
                min_x,
                min_y,
                max_x,
                max_y,
                scale_x,
                scale_y,
                range_x,
                range_y,
                width,
                height,
            }
        }

        pub fn x_coord_to_pixel(&self, coord: f64) -> f64 {
            GRID_MARGIN + (coord - self.min_x) * self.scale_x
        }

        pub fn y_coord_to_pixel(&self, coord: f64) -> f64 {
            // invert y-axis for pixel coordinates
            self.height - GRID_MARGIN - (coord - self.min_y) * self.scale_y
        }
    }

    fn draw_cartesian_grid(cr: &cairo::Context, ctx: &ScaleCtx) {
        cr.set_source_rgb(0.8, 0.8, 0.8);
        cr.set_line_width(1.0);

        // Vertical lines
        for i in 0..=NUM_SQUARES {
            let ratio = i as f64 / NUM_SQUARES as f64;
            let x_coord = ctx.min_x + ratio * ctx.range_x;
            let x_pixel = ctx.x_coord_to_pixel(x_coord);

            cr.move_to(x_pixel, GRID_MARGIN);
            cr.line_to(x_pixel, ctx.height - GRID_MARGIN);
            cr.stroke().expect("Failed to draw vertical line");

            let label = format!("{:.2}", x_coord);
            // the / 2.0 makes the text a little tighter to bottom of grid
            cr.move_to(x_pixel - TEXT_PADDING, ctx.height - GRID_MARGIN / 2.0);
            cr.show_text(&label).expect("Failed to draw text");
        }

        // Horizontal lines
        for i in 0..=NUM_SQUARES {
            let ratio = i as f64 / NUM_SQUARES as f64;
            let y_coord = ctx.min_y + ratio * ctx.range_y;
            let y_pixel = ctx.y_coord_to_pixel(y_coord);

            cr.move_to(GRID_MARGIN, y_pixel);
            cr.line_to(ctx.width - GRID_MARGIN, y_pixel);
            cr.stroke().expect("Failed to draw horizontal line");

            let label = format!("{:.2}", y_coord);
            cr.move_to(TEXT_PADDING, y_pixel);
            cr.show_text(&label).expect("Failed to draw text");
        }
    }
    fn battery_colors(battery: f64) -> (f64, f64, f64) {
        let battery = battery.clamp(0.0, 100.0);
        if battery == 0.0 {
            (0.0, 0.0, 0.0)
        } else if battery <= 1.0 {
            (0.5, 0.0, 0.0)
        } else {
            let t = battery / 100.0;
            (1.0 - t, t, 0.0)
        }
    }

    fn draw_nodes(cr: &cairo::Context, ctx: &ScaleCtx, nodes: &HashMap<String, Node>) {
        for node in nodes.values() {
            let x_pixel = ctx.x_coord_to_pixel(node.coords.x);
            let y_pixel = ctx.y_coord_to_pixel(node.coords.y);

            let (r, g, b) = battery_colors(node.battery);
            cr.set_source_rgb(r, g, b);
            cr.arc(
                x_pixel,
                y_pixel,
                NODE_RADIUS,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            cr.fill().expect("Failed to fill circle");
        }
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
