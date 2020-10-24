use crate::ray::{HitRecord, Hittable, Ray};
use crate::vec::{self, Point};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let c_to_o = ray.origin - self.center;
        let a = ray.direction.length_squared();
        // b has a factor 2 so let b = 2h
        // the quadratic equation is t = (-b +- sqrt(b² - 4ac)) / 2a
        // replacing b gives (-2h +- sqrt((2h)² - 4ac)) / 2a
        // then              (-h +- sqrt(h² - ac)) / a
        let half_b = vec::dot(&ray.direction, &c_to_o);
        let c = c_to_o.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let discr_sqrt = discriminant.sqrt();
            let mut t = (-half_b - discr_sqrt) / a;
            fn within_range(t: f64, min: f64, max: f64) -> bool {
                t > min && t < max
            }
            if !within_range(t, t_min, t_max) {
                t = (-half_b + discr_sqrt) / a;
                if !within_range(t, t_min, t_max) {
                    return None;
                }
            }
            let intersect = ray.at(t);
            let normal = intersect - self.center;
            Some(HitRecord::new(intersect, vec::unit(&normal), t))
        }
    }
}
