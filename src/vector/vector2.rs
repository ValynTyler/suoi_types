use std::ops::Sub;

use crate::Vector;

#[derive(Debug, Clone, Copy)]
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

    fn dot(&self, _rhs: Self) -> f32 {
        todo!()
    }

    fn len(&self) -> f32 {
        todo!()
    }

    fn list(&self) -> Vec<f32> {
        vec![self.x, self.y]
    }
}

// Arithmetic
impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Methods
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
