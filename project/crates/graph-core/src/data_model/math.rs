#[derive(Debug, Clone, Copy)]
pub struct IntCoords2D {
    x: i32,
    y: i32,
}

impl IntCoords2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // [INSTANCE FUNCTIONS]

    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        let dx = f64::from(other.x) - f64::from(self.x);
        let dy = f64::from(other.y) - f64::from(self.y);

        dx.hypot(dy)
    }
}
