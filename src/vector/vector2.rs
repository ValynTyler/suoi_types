use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::zero()
    }
}

// Vector
impl Vector for Vector2 {
    fn zero() -> Self {
        Self { x: 0., y: 0. }
    }

    fn one() -> Self {
        Self { x: 1., y: 1. }
    }

    fn dot(&self, _rhs: Self) -> f32 {
        todo!()
    }

    fn len(&self) -> f32 {
        todo!()
    }

    fn list(&self) -> Vec<f32> {
        vec![self.x, self.y]
    }

    fn unit(&self) -> Self {
        Self {
            x: self.x * 1.0 / self.len(),
            y: self.y * 1.0 / self.len(),
        }
    }
}

// Arithmetic
impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Display
impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "[{:.*}, {:.*}]", precision, self.x, precision, self.y)
        } else {
            write!(f, "[{}, {}]", self.x, self.y)
        }
    }
}

// Methods
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
