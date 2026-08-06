pub struct Generator {
    pub x: u8,
    pub y: u8,
    pub walkers: u8,
}
pub struct World {
    x: Vec<f32>,
    y: Vec<f32>,
    walkers: u8,
}

impl World {
    pub fn new(x: Vec<f32>, y: Vec<f32>, walkers: u8) -> Self {
        Self { x, y, walkers }
    }
}
