use crate::image::Image;
use std::io;

pub struct PPMWriter<W: io::Write> {
    writer: W,
}

impl<W: io::Write> PPMWriter<W> {
    pub fn new(writer: W) -> Self {
        PPMWriter { writer }
    }

    pub fn write(&mut self, img: &Image) -> io::Result<()> {
        self.writer.write_all(b"P3\n")?;
        self.writer
            .write_all(format!("{} {}\n", img.width, img.height).as_bytes())?;
        self.writer.write_all(b"255\n")?;
        for l in 0..img.height {
            for c in 0..img.width {
                let px = &img.data[l * img.height + c];
                self.writer
                    .write_all(format!("{} {} {} ", px.red, px.green, px.blue).as_bytes())?;
            }
            self.writer.write_all(b"\n")?;
        }
        Ok(())
    }
}
