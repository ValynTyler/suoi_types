use std::ops::{Add, Mul};

use nerd::vector::Vector3;

#[derive(Debug)]
pub struct Quaternion {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

// Conversions
impl From<Vector3> for Quaternion {
    fn from(value: Vector3) -> Self {
        Quaternion::new(0.0, value.x, value.y, value.z)
    }
}

// Methods
impl Quaternion {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }

    /// returns the real part of `self`
    pub fn re(&self) -> f32 {
        self.scalar_part()
    }

    /// returns the imaginary part of `self`
    pub fn im(&self) -> Vector3 {
        self.vector_part()
    }

    pub fn scalar_part(&self) -> f32 {
        self.a
    }

    pub fn vector_part(&self) -> Vector3 {
        Vector3 {
            x: self.b,
            y: self.c,
            z: self.d,
        }
    }
}

// Algebra
impl Add<f32> for Quaternion {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            a: self.a + rhs,
            ..self
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (a1, a2) = (self.a, rhs.a);
        let (b1, b2) = (self.b, rhs.b);
        let (c1, c2) = (self.c, rhs.c);
        let (d1, d2) = (self.d, rhs.d);
        Self {
            a: a1*a2 - b1*b2 - c1*c2 - d1*d2,
            b: a1*b2 + b1*a2 + c1*d2 - d1*c2,
            c: a1*c2 + c1*a2 - b1*d2 + d1*b2,
            d: a1*d2 + d1*a2 + b1*c2 - c1*b2,
        }
    }
}
