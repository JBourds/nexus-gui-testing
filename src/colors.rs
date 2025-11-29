pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

pub fn battery_colors(charge: f64) -> Rgb {
    let charge = charge.clamp(0.0, 1.0);
    if charge == 0.0 {
        return Rgb::black();
    }
    Rgb::new(1.0 - charge, charge, 0.0)
}
