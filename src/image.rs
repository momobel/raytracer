use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn clamp(&mut self, min: f64, max: f64) {
        self.red = clamp(self.red, min, max);
        self.green = clamp(self.green, min, max);
        self.blue = clamp(self.blue, min, max);
    }
}

pub mod colors {
    use super::*;
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
}

impl std::default::Default for Color {
    fn default() -> Self {
        colors::BLACK
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, other: &Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        &self + &other
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, color: &Color) -> Color {
        Color {
            blue: color.blue * self,
            green: color.green * self,
            red: color.red * self,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        self * &color
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Div<f64> for &Color {
    type Output = Color;

    fn div(self, val: f64) -> Color {
        (1.0 / val) * self
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let sz: usize = width * height;
        let mut data: Vec<Color> = Vec::with_capacity(sz);
        for _ in 0..sz {
            data.push(Color::default());
        }
        Image {
            width,
            height,
            data,
        }
    }
}
