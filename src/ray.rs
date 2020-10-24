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
