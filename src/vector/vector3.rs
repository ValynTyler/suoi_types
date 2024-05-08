use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub},
};

use crate::{Quaternion, Vector};

#[derive(Clone, Copy)]
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
}

// Arithmetic
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

    pub fn rotate(&mut self, rotation: Quaternion) -> Self {
        let q = rotation;
        (Quaternion::from(q * Vector3::fwd()) * q.recip()).into()
    }
}
