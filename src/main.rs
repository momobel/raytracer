use rand::{self, distributions::Distribution, Rng};
use std::fs;
use std::io::{self, Write};
use structopt::StructOpt;
mod image;
mod material;
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
    u: Vector,
    v: Vector,
}

impl Camera {
    pub fn new(
        position: Point,
        look_at: Point,
        vup: Vector,
        vert_fov: f64,
        aspect_ratio: f64,
        focal: f64,
    ) -> Self {
        let w = vec::unit(&(position - look_at));
        let u = vec::unit(&vec::cross(&vup, &w));
        let v = vec::cross(&w, &u);
        let height = 2.0 * (vert_fov.to_radians() / 2.0).tan();
        let viewport = Viewport::new(aspect_ratio * height, height);
        let horizontal = viewport.width * u;
        let vertical = viewport.height * v;
        let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - focal * w;
        Self {
            position,
            viewport,
            focal,
            lower_left_corner,
            u,
            v,
        }
    }

    pub fn ray(&self, t: f64, s: f64) -> Ray {
        Ray::new(
            self.position,
            self.lower_left_corner + t * &self.u + s * &self.v - self.position,
        )
    }
}

#[derive(Debug)]
struct RenderSettings {
    pub antialiasing_samples: u16,
    pub ray_bounce_limit: u16,
    pub gamma: f64,
}

impl std::default::Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            antialiasing_samples: 1,
            ray_bounce_limit: 0,
            gamma: 1.0,
        }
    }
}

impl RenderSettings {
    pub fn aa_samples(&mut self, val: u16) -> &mut Self {
        self.antialiasing_samples = val;
        self
    }
    pub fn ray_bounce_limit(&mut self, val: u16) -> &mut Self {
        self.ray_bounce_limit = val;
        self
    }
    pub fn gamma(&mut self, val: u16) -> &mut Self {
        self.gamma = 1.0 / val as f64;
        self
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
    let vert_fov = 90.0;
    let focal_length = 1.0;
    let origin = Point::new(-2.0, 2.0, 1.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let camera = Camera::new(origin, look_at, vup, vert_fov, aspect_ratio, focal_length);
    // world
    let material_ground = material::Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = material::Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = material::Dielectric::new(1.5);
    let material_right = material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let world = HittableVec::new(vec![
        Sphere::new(
            Point::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(material_ground),
        ),
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, Box::new(material_center)),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, Box::new(material_left)),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.4, Box::new(material_left)),
        Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, Box::new(material_right)),
    ]);
    // render
    let mut settings = RenderSettings::default();
    settings.aa_samples(100).ray_bounce_limit(50).gamma(2);
    fill_image(&mut img, &settings, &camera, &world);
    let file =
        fs::File::create(&opt.output).expect(format!("Failed to open {}", opt.output).as_str());
    let mut writer: ppm::PPMWriter<fs::File> = ppm::PPMWriter::new(file);
    writer.write(&img).expect("Failed to write image");
}

fn random_in_hemisphere(normal: &Vector) -> Vector {
    let random_unit = vec::random_unit_vector();
    if vec::dot(&random_unit, normal) > 0.0 {
        random_unit
    } else {
        -random_unit
    }
}

fn ray_color(ray: &Ray, world: &HittableVec<Sphere>, depth: i16) -> Color {
    // ray bounced too many times, no more light is gathered
    if depth < 0 {
        return image::colors::BLACK;
    }
    if let Some(hit) = world.hit_by(ray, 0.001, ray::T_INFINITY) {
        let effect = hit.material.scatter(ray, &hit);
        match effect.scattered {
            None => return image::colors::BLACK,
            Some(scattered) => return effect.attenuation * ray_color(&scattered, world, depth - 1),
        }
    }
    let unit_dir = vec::unit(&ray.direction);
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn fill_image(
    img: &mut image::Image,
    settings: &RenderSettings,
    camera: &Camera,
    world: &HittableVec<Sphere>,
) {
    let range_rand = rand::distributions::Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    let samples = settings.antialiasing_samples;
    for line in 0..img.height {
        eprint!("\rLines remaining: {:3}", img.height - line);
        io::stderr().flush().unwrap();
        for col in 0..img.width {
            let px = &mut img.data[line * img.width + col];
            let mut color = image::colors::BLACK;
            for _ in 0..samples {
                let u = (col as f64 + range_rand.sample(&mut rng)) / (img.width as f64 - 1.0);
                // render starts on top left
                let v = (img.height as f64 - (line as f64 + range_rand.sample(&mut rng)))
                    / (img.height as f64 - 1.0);
                let ray = camera.ray(u, v);
                color = color + ray_color(&ray, world, settings.ray_bounce_limit as i16);
            }
            // gamma correction
            // gamma G means raising the color to the power 1/G
            color = &color / samples as f64;
            color.red = color.red.powf(settings.gamma);
            color.green = color.green.powf(settings.gamma);
            color.blue = color.blue.powf(settings.gamma);
            color.clamp(0.0, 0.999);
            *px = color;
        }
    }
}
