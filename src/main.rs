use std::fs;
use structopt::StructOpt;
mod image;
mod ppm;
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

#[derive(Debug)]
struct Sphere {
    pub center: vec::Point,
    pub radius: f64,
}

fn hit_sphere(ray: &vec::Ray, sphere: &Sphere) -> Option<f64> {
    // let S be a sphere of center C and radius r
    // a point P is on the sphere if ||P - C||² = r²
    // a vector V has ||V||² = V.V
    // a ray R with origin O and direction D hits the sphere
    // if for any t ||O + tD - C||² = r²
    //               (O + tD - C).(O + tD - C) = r²
    // which means t²||D||² + 2tD.(O - C) + ||O - C||² - r² = 0
    // This is a quadratic equation with
    // a = ||D||²
    // b = 2D.(O-C)
    // c = ||O - C||² - r²
    // discriminant d is b² - 4ac
    // if negative, no real solution exist so no intersection
    // if 0, a single solution exists -b / 2a
    // if positive, 2 solutions exist (-b +- sqrt(d)) / 2a
    let c_to_o = ray.origin - sphere.center;
    let a = ray.direction.length_squared();
    let b = 2.0 * vec::dot(&ray.direction, &c_to_o);
    let c = c_to_o.length_squared() - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

const SCENE_SPHERE: Sphere = Sphere {
    center: vec::Point {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    },
    radius: 0.5,
};

fn ray_color(ray: &vec::Ray) -> image::Color {
    if let Some(t) = hit_sphere(ray, &SCENE_SPHERE) {
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
            let ray = vec::Ray::new(origin, &dir);
            *px = ray_color(&ray);
        }
    }
}
