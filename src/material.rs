use crate::image::Color;
use crate::ray::{HitRecord, Ray};
use crate::vec;

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
