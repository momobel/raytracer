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
    #[structopt(short, long, default_value = "1200")]
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
    position: Point,
    viewport: Viewport,
    focal: f64,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        position: Point,
        look_at: Point,
        vup: Vector,
        vert_fov: f64,
        aspect_ratio: f64,
        focal: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let w = vec::unit(&(position - look_at));
        let u = vec::unit(&vec::cross(&vup, &w));
        let v = vec::cross(&w, &u);
        let height = 2.0 * (vert_fov.to_radians() / 2.0).tan();
        let viewport = Viewport::new(aspect_ratio * height, height);
        let horizontal = focus_dist * viewport.width * u;
        let vertical = focus_dist * viewport.height * v;
        let lower_left_corner =
            position - horizontal / 2.0 - vertical / 2.0 - focal * focus_dist * w;
        Self {
            position,
            viewport,
            focal,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, t: f64, s: f64) -> Ray {
        let rd = self.lens_radius * vec::random_in_unit_disk();
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(
            self.position + offset,
            self.lower_left_corner + t * &self.horizontal + s * &self.vertical
                - self.position
                - offset,
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
    let aspect_ratio = 3.0 / 2.0;
    let opt = Options::from_args();
    // image
    let mut img = image::Image::new(
        opt.width as usize,
        (opt.width as f64 / aspect_ratio) as usize,
    );
    // camera
    let vert_fov = 20.0;
    let focal_length = 1.0;
    let origin = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        origin,
        look_at,
        vup,
        vert_fov,
        aspect_ratio,
        focal_length,
        aperture,
        dist_to_focus,
    );
    // world
    let mut spheres = vec![
        Sphere::new(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(material::Lambertian::new(Color::new(0.5, 0.5, 0.5))),
        ),
        Sphere::new(
            Point::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(material::Dielectric::new(1.5)),
        ),
        Sphere::new(
            Point::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1))),
        ),
        Sphere::new(
            Point::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
        ),
    ];
    let refp = Point::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let center = Point::new(
                a as f64 + 0.9 * random_unit(),
                0.2,
                b as f64 + 0.9 * random_unit(),
            );
            if (center - refp).length() > 0.9 {
                let rd_material = random_unit();
                let material: Box<dyn material::Material> = if rd_material < 0.8 {
                    let albedo = random_color() * random_color();
                    Box::new(material::Lambertian::new(albedo))
                } else if rd_material < 0.95 {
                    let albedo = random_color_ranged(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    Box::new(material::Metal::new(albedo, fuzz))
                } else {
                    Box::new(material::Dielectric::new(1.5))
                };
                let sphere = Sphere::new(center, 0.2, material);
                spheres.push(sphere);
            }
        }
    }
    let world = HittableVec::new(spheres);
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

fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}

fn random_unit() -> f64 {
    random_range(0.0, 1.0)
}

fn random_color() -> Color {
    Color::new(random_unit(), random_unit(), random_unit())
}

fn random_color_ranged(min: f64, max: f64) -> Color {
    Color::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}
