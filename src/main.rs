use rand::{self, distributions::Distribution};
use std::fs;
use std::io::{self, Write};
use structopt::StructOpt;
mod image;
mod ppm;
mod ray;
mod sphere;
mod vec;
use image::Color;
use ray::{HittableVec, Ray};
use sphere::Sphere;
use vec::{Point, Vector};

#[derive(StructOpt, Debug)]
#[structopt(name = "ray")]
struct Options {
    #[structopt(short, long, default_value = "400")]
    width: u16,
    output: String,
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

#[derive(Debug)]
struct Camera {
    pub position: Point,
    pub viewport: Viewport,
    pub focal: f64,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(position: Point, viewport: Viewport, focal: f64) -> Self {
        let horizontal = Vector::new(viewport.width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport.height, 0.0);
        let lower_left_corner =
            position - horizontal / 2.0 - vertical / 2.0 + Vector::new(0.0, 0.0, -focal);
        Self {
            position,
            viewport,
            focal,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.position,
            self.lower_left_corner + u * &self.horizontal + v * &self.vertical - self.position,
        )
    }
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
    let origin = Point::new(0.0, 0.0, 0.0);
    let camera = Camera::new(origin, viewport, focal_length);
    // world
    let world = HittableVec::new(vec![
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0),
    ]);
    // render
    fill_image(&mut img, &camera, &world);
    let file =
        fs::File::create(&opt.output).expect(format!("Failed to open {}", opt.output).as_str());
    let mut writer: ppm::PPMWriter<fs::File> = ppm::PPMWriter::new(file);
    writer.write(&img).expect("Failed to write image");
}

fn ray_color(ray: &Ray, world: &HittableVec<Sphere>) -> Color {
    if let Some(hit) = world.hit_by(ray, 0.0, ray::T_INFINITY) {
        return 0.5 * Color::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0);
    }
    let unit_dir = vec::unit(&ray.direction);
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn fill_image(img: &mut image::Image, camera: &Camera, world: &HittableVec<Sphere>) {
    let range_rand = rand::distributions::Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let samples: u8 = 100;
    for line in 0..img.height {
        eprint!("\rLines remaining: {:3}", img.height - line);
        io::stderr().flush().unwrap();
        for col in 0..img.width {
            let px = &mut img.data[line * img.width + col];
            let mut color = image::colors::BLACK;
            for _ in 0..samples {
                let u = (col as f64 + range_rand.sample(&mut rng)) / (img.width as f64 - 1.0);
                let v = (img.height as f64 - (line as f64 + range_rand.sample(&mut rng)))
                    / (img.height as f64 - 1.0);
                let ray = camera.ray(u, v);
                color = color + ray_color(&ray, world);
            }
            *px = &color / samples as f64;
        }
    }
}
