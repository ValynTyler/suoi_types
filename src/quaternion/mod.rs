use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use nerd::vector::Vector3;

use crate::Angle;
use crate::Matrix4;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Into<Matrix4> for Quaternion {
    fn into(self) -> Matrix4 {
        self.mat()
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

    #[rustfmt::skip]
    pub fn conj(&self) -> Self {
        Self {
            a:  self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d,
        }
    }

    pub fn norm(&self) -> f32 {
        (*self * self.conj()).a
    }

    pub fn recip(&self) -> Self {
        Self {
            a: (self.conj().a) / (self.norm() * self.norm()),
            b: (self.conj().b) / (self.norm() * self.norm()),
            c: (self.conj().c) / (self.norm() * self.norm()),
            d: (self.conj().d) / (self.norm() * self.norm()),
        }
    }

    pub fn axis_angle<A>(axis: Vector3, angle: A) -> Self
    where
        A: Angle,
    {
        Quaternion::from(axis * (angle.rad().0 / 2.).sin()) + (angle.rad().0 / 2.).cos()
    }

    #[rustfmt::skip]
    pub fn mat(&self) -> Matrix4 {
        let q = self;
        let (r, i, j, k) = (q.a, q.b, q.c, q.d);
        Matrix4(
            [1. - 2.*(j*j + k*k),    2.*(i*j - k*r),         2.*(i*k + j*r),         0.],
            [2.*(i*j + k*r),         1. - 2.*(i*i + k*k),    2.*(j*k - i*r),         0.],
            [2.*(i*k - j*r),         2.*(j*k + i*r),         1. - 2.*(i*i + j*j),    0.],
            [0.,                     0.,                     0.,                     1.],
        )
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

// Display
impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.a, self.b, self.c, self.d)?;
        Ok(())
    }
}

// Outside implementations
pub trait Rotate {
    fn rotate(&mut self, rotation: Quaternion) -> Self;
}

impl Rotate for Vector3 {
    fn rotate(&mut self, rotation: Quaternion) -> Self {
        let q = rotation;
        (Quaternion::from(q * Vector3::FORWARD) * q.recip()).into()
    }
}
