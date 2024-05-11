use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    pub const WHITE: Self = Self {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    pub const RED: Self = Self {
        r: 1.,
        g: 0.,
        b: 0.,
    };
    pub const GREEN: Self = Self {
        r: 0.,
        g: 1.,
        b: 0.,
    };
    pub const BLUE: Self = Self {
        r: 0.,
        g: 0.,
        b: 1.,
    };
    pub const YELLOW: Self = Self {
        r: 1.,
        g: 1.,
        b: 0.,
    };
    pub const CYAN: Self = Self {
        r: 0.,
        g: 1.,
        b: 1.,
    };
    pub const MAGENTA: Self = Self {
        r: 1.,
        g: 0.,
        b: 1.,
    };
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        let r = (color.r * 255.).min(255.).max(0.) as u32;
        let g = (color.g * 255.).min(255.).max(0.) as u32;
        let b = (color.b * 255.).min(255.).max(0.) as u32;

        r << 16 | g << 8 | b
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
