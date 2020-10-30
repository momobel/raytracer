use crate::image::Color;
use crate::ray::{HitRecord, Ray};
use crate::vec::{self, Vector};
use rand::{self, Rng};

pub struct MaterialEffect {
    pub attenuation: Color,
    pub scattered: Option<Ray>,
}

impl std::default::Default for MaterialEffect {
    fn default() -> Self {
        Self {
            attenuation: Color::new(0.0, 0.0, 0.0),
            scattered: None,
        }
    }
}

impl MaterialEffect {
    pub fn new(attenuation: Color, scatter: Ray) -> Self {
        Self {
            attenuation,
            scattered: Some(scatter),
        }
    }
    pub fn with_attenuation(attenuation: Color) -> Self {
        Self {
            attenuation,
            scattered: None,
        }
    }
}

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> MaterialEffect;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> MaterialEffect {
        let scatter_dir = hit.normal + vec::random_unit_vector();
        let scattered = Ray::new(hit.point, scatter_dir);
        MaterialEffect::new(self.albedo, scattered)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuziness: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuziness < 1.0 { fuziness } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> MaterialEffect {
        let reflected = vec::reflect(&ray.direction, &hit.normal);
        if vec::dot(&reflected, &hit.normal) > 0.0 {
            let scattered = Ray::new(
                hit.point,
                reflected + self.fuzz * &vec::random_unit_vector(),
            );
            MaterialEffect::new(self.albedo, scattered)
        } else {
            MaterialEffect::with_attenuation(self.albedo)
        }
    }
}

fn refract(incoming: &Vector, normal: &Vector, etai_over_etat: f64) -> Vector {
    // cos_theta = dot(-incoming, normal)
    let r_perp = etai_over_etat * (incoming + vec::dot(&-incoming, normal) * normal);
    let r_par = -(1.0 - r_perp.length_squared()).abs().sqrt() * normal;
    r_perp + r_par
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> MaterialEffect {
        let no_attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = vec::unit(&ray.direction);
        // cos(theta) = -R . n
        let cos_theta = vec::dot(&-unit_dir, &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        // n/n' sin(theta) = sin(theta')
        // sin(theta') <= 1 to refract so n/n' sin(theta) < 1
        // otherwise it reflects
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let rand_f64 = rand::thread_rng().gen_range(0.0, 1.0);
        let new_ray_dir = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand_f64 {
            vec::reflect(&unit_dir, &hit.normal)
        } else {
            refract(&unit_dir, &hit.normal, refraction_ratio)
        };
        MaterialEffect::new(no_attenuation, Ray::new(hit.point, new_ray_dir))
    }
}

fn reflectance(cos: f64, refr_ratio: f64) -> f64 {
    let mut r0 = (1.0 - refr_ratio) / (1.0 + refr_ratio);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
