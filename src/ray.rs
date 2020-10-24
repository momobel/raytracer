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
}

impl HitRecord {
    pub fn new(point: Point, normal: Vector, t: f64) -> Self {
        Self { point, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
