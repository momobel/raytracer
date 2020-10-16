use std::fs;
use structopt::StructOpt;
mod image;
mod ppm;

#[derive(StructOpt, Debug)]
#[structopt(name = "ray")]
struct Options {
    #[structopt(short, long, default_value = "255")]
    width: u16,
    #[structopt(short, long, default_value = "255")]
    height: u16,
    output: String,
}

fn fill_image(img: &mut image::Image) {
    for line in 0..img.height {
        for col in 0..img.width {
            let px = &mut img.data[line * img.height + col];
            px.red = line as f64 / img.height as f64;
            px.blue = col as f64 / img.width as f64;
        }
    }
}

fn main() {
    let opt = Options::from_args();
    let mut img = image::Image::new(opt.width as usize, opt.height as usize);
    fill_image(&mut img);
    let file =
        fs::File::create(&opt.output).expect(format!("Failed to open {}", opt.output).as_str());
    let mut writer: ppm::PPMWriter<fs::File> = ppm::PPMWriter::new(file);
    let res = writer.write(&img).expect("Failed to write image");
}
