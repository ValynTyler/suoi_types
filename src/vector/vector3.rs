use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Neg, Sub},
};

use crate::{Quaternion, Vector, Vector2};

#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Default
impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

// Vector
impl Vector for Vector3 {
    fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    fn one() -> Self {
        Self {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }

    fn len(&self) -> f32 {
        let mut s = 0.0;
        for elem in self.list() {
            s += elem * elem;
        }
        s.sqrt()
    }

    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn list(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }

    fn unit(&self) -> Self {
        Self {
            x: self.x * 1.0 / self.len(),
            y: self.y * 1.0 / self.len(),
            z: self.z * 1.0 / self.len(),
        }
    }
}

// Arithmetic
impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// Display
impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)?;
        Ok(())
    }
}

// Methods
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn right() -> Self {
        Self {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn up() -> Self {
        Self {
            x: 0.,
            y: 1.,
            z: 0.,
        }
    }

    pub fn fwd() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 1.,
        }
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn angle(&self, other: Vector3) -> f32 {
        f32::acos((self.dot(other)) / (self.len() * other.len()))
    }

    pub fn rotate(&mut self, rotation: Quaternion) -> Self {
        let q = rotation;
        (Quaternion::from(q * Vector3::fwd()) * q.recip()).into()
    }

    pub fn xy(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn xz(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.z,
        }
    }

    pub fn yz(&self) -> Vector2 {
        Vector2 {
            x: self.y,
            y: self.z,
        }
    }
}
