use crate::vec;

#[derive(Debug)]
pub struct Ray {
    pub origin: vec::Point,
    pub direction: vec::Vector,
}

impl Ray {
    pub fn new(origin: &vec::Point, direction: &vec::Vector) -> Ray {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn at(&self, t: f64) -> vec::Point {
        self.origin + t * &self.direction
    }
}
