use crate::vec::{Point, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: &Point, direction: &Vector) -> Ray {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * &self.direction
    }
}

#[derive(Debug)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point, normal: Vector, t: f64, front_face: bool) -> Self {
        let normal = if front_face { normal } else { -normal };
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableVec<T: Hittable> {
    vec: Vec<T>,
}

impl<T: Hittable> HittableVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec }
    }

    pub fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut hit: Option<HitRecord> = None;
        for item in &self.vec {
            if let Some(h) = item.hit_by(ray, t_min, closest) {
                closest = h.t;
                hit = Some(h);
            }
        }
        hit
    }
}

pub const T_INFINITY: f64 = f64::MAX;
