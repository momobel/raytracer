use std::fs;
use structopt::StructOpt;
mod image;
mod ppm;
mod ray;
mod sphere;
mod vec;

#[derive(StructOpt, Debug)]
#[structopt(name = "ray")]
struct Options {
    #[structopt(short, long, default_value = "400")]
    width: u16,
    output: String,
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let opt = Options::from_args();
    // image
    let mut img = image::Image::new(
        opt.width as usize,
        (opt.width as f64 / aspect_ratio) as usize,
    );
    // camera
    let viewport_height = 2.0;
    let viewport = Viewport::new(aspect_ratio * viewport_height, viewport_height);
    let focal_length = 1.0;
    let origin = vec::Point::new(0.0, 0.0, 0.0);
    // render
    fill_image(&mut img, &origin, &viewport, focal_length);
    let file =
        fs::File::create(&opt.output).expect(format!("Failed to open {}", opt.output).as_str());
    let mut writer: ppm::PPMWriter<fs::File> = ppm::PPMWriter::new(file);
    writer.write(&img).expect("Failed to write image");
}

#[derive(Debug)]
struct Viewport {
    pub width: f64,
    pub height: f64,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

const SCENE_SPHERE: sphere::Sphere = sphere::Sphere {
    center: vec::Point {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    },
    radius: 0.5,
};

fn ray_color(ray: &ray::Ray) -> image::Color {
    if let Some(t) = sphere::hit_sphere(ray, &SCENE_SPHERE) {
        let intersection = ray.at(t);
        let normal = intersection - SCENE_SPHERE.center;
        let normal = vec::unit(&normal);
        return 0.5 * image::Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }
    let unit_dir = vec::unit(&ray.direction);
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * image::Color::new(1.0, 1.0, 1.0) + t * image::Color::new(0.5, 0.7, 1.0)
}

fn fill_image(img: &mut image::Image, origin: &vec::Point, view: &Viewport, focal_length: f64) {
    let horizontal = vec::Vector::new(view.width, 0.0, 0.0);
    let vertical = vec::Vector::new(0.0, view.height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec::Vector::new(0.0, 0.0, focal_length);

    for line in 0..img.height {
        for col in 0..img.width {
            let px = &mut img.data[line * img.width + col];
            let u = col as f64 / (img.width - 1) as f64;
            let v = (img.height - line) as f64 / (img.height - 1) as f64;
            let dir = lower_left_corner + u * &horizontal + v * &vertical - origin;
            let ray = ray::Ray::new(origin, &dir);
            *px = ray_color(&ray);
        }
    }
}
