use std::ops::{Add, Mul};

use nerd::{matrix::Matrix4, vector::Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

// Conversions
impl From<Vector3> for Quaternion {
    fn from(value: Vector3) -> Self {
        Quaternion::new(0.0, value.x, value.y, value.z)
    }
}

impl Into<Vector3> for Quaternion {
    fn into(self) -> Vector3 {
        self.vector_part()
    }
}

#[rustfmt::skip]
impl Into<Matrix4> for Quaternion {
    fn into(self) -> Matrix4 {
        let (w, x, y, z) = (self.a, self.b, self.c, self.d);
        Matrix4([
             w,  z, -y,  x,
            -z,  w,  x,  y,
             y, -x,  w,  z,
            -x, -y, -z,  w,
        ]) * Matrix4([
            w,  z, -y, -x,
           -z,  w,  x, -y,
            y, -x,  w, -z,
            x,  y,  z,  w,
        ])

    }
}

// Methods
impl Quaternion {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }

    /// returns the length of `self`
    pub fn len(&self) -> f32 {
        self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d
    }

    /// returns the real part of `self`
    pub fn re(&self) -> f32 {
        self.scalar_part()
    }

    /// returns the imaginary part of `self`
    pub fn im(&self) -> Vector3 {
        self.vector_part()
    }

    /// returns the scalar part of `self`
    pub fn scalar_part(&self) -> f32 {
        self.a
    }

    /// returns the vector part of `self`
    pub fn vector_part(&self) -> Vector3 {
        Vector3 {
            x: self.b,
            y: self.c,
            z: self.d,
        }
    }

    pub fn axis_angle(axis: Vector3, angle: f32) -> Self {
        Quaternion::from(axis * (angle / -2.).sin()) + (angle / -2.).cos()
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
            a: a1 * a2 - b1 * b2 - c1 * c2 - d1 * d2,
            b: a1 * b2 + b1 * a2 + c1 * d2 - d1 * c2,
            c: a1 * c2 + c1 * a2 - b1 * d2 + d1 * b2,
            d: a1 * d2 + d1 * a2 + b1 * c2 - c1 * b2,
        }
    }
}

impl Mul<Vector3> for Quaternion {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        (self * Self::from(rhs)).into()
    }
}
