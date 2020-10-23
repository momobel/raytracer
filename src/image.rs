use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl std::default::Default for Color {
    fn default() -> Self {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
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
