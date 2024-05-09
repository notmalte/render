pub struct Viewport {
    width: f64,
    height: f64,
    distance: f64,
}

impl Viewport {
    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1.,
            height: 1.,
            distance: 1.,
        }
    }
}
