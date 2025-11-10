pub const NODES: [Node; 5] = [
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
