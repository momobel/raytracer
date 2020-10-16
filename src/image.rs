pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
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
