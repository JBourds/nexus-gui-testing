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
