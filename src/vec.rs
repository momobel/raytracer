use rand::{self, Rng};
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        -&self
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        &self + other
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        self + &other
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        &self - other
    }
}

impl Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        self - &other
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        &self - &other
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        &self * rhs
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Vector {
        self * &vec
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        rhs * self
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        &self / rhs
    }
}

pub fn dot(a: &Vector, b: &Vector) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    Vector {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn unit(v: &Vector) -> Vector {
    v / v.length()
}

pub fn random_unit_vector() -> Vector {
    // by fixing one coordinate and an angle
    let teta: f64 = rand::thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z: f64 = rand::thread_rng().gen_range(-1.0, 1.0);
    // a unit vector has equation x² + y² + z² = 1
    // thus x² + y² = 1 - z², given x² + y² = Rxy²
    // with Rxy the radius of circle at "height" z
    let r: f64 = (1.0 - z * z).sqrt();
    Vector::new(r * teta.cos(), r * teta.sin(), z)
}
pub fn random_in_unit_disk() -> Vector {
    let x = rand::thread_rng().gen_range(-1.0, 1.0);
    let x_square = x * x;
    let y = loop {
        let guess = rand::thread_rng().gen_range(0.0, 1.0);
        if x_square + guess * guess < 1.0 {
            break guess;
        }
    };
    Vector::new(x, y, 0.0)
}

pub fn reflect(v: &Vector, normal: &Vector) -> Vector {
    v - 2.0 * dot(v, normal) * normal
}

pub type Point = Vector;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn length_example() {
        assert_eq!(6.0, Vector::new(2.0, 4.0, 4.0).length());
    }
    #[test]
    fn dot_example() {
        let a = Vector::new(1.0, 5.0, 3.0);
        let b = Vector::new(1.0, 3.0, 3.0);
        assert_eq!(25.0, dot(&a, &b));
    }
    #[test]
    fn cross_example() {
        let u = Vector::new(2., 3., 4.);
        let v = Vector::new(5., 6., 7.);
        assert_eq!(Vector::new(-3., 6., -3.), cross(&u, &v))
    }
}
