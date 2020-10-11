pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl std::default::Default for Pixel {
    fn default() -> Self {
        Pixel {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let sz: usize = width * height;
        let mut data: Vec<Pixel> = Vec::with_capacity(sz);
        for _ in 0..sz {
            data.push(Pixel::default());
        }
        Image {
            width,
            height,
            data,
        }
    }
}
