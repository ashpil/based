use std::ops::{Add, AddAssign, Mul, Sub, Div};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}


impl Color {
    pub const BLACK: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
    pub const WHITE: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn write_16(self, buffer: &mut [u8]) {
        self.write_16_sampled(buffer, 1);
    }

    pub fn write_16_sampled(self, buffer: &mut [u8], num_samples: u16) {
        let write_color = self / num_samples as f64;
        let red = (write_color.r.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
        let green = (write_color.g.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
        let blue = (write_color.b.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
        buffer[0] = (red >> 8) as u8;
        buffer[1] = red as u8;
        buffer[2] = (green >> 8) as u8;
        buffer[3] = green as u8;
        buffer[4] = (blue >> 8) as u8;
        buffer[5] = blue as u8;
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self::Output {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, s: f64) -> Self::Output {
        Color {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, col: Color) -> Self::Output {
        Color {
            r: col.r * self,
            g: col.g * self,
            b: col.b * self,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, s: f64) -> Self::Output {
        Color {
            r: self.r / s,
            g: self.g / s,
            b: self.b / s,
        }
    }
}

