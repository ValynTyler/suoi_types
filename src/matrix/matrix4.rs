use std::{fmt::Display, ops::Mul};

use crate::Matrix;
use crate::Vector;
use crate::Vector3;

#[allow(unused)]
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4(
    pub[f32; 4],
    pub[f32; 4],
    pub[f32; 4],
    pub[f32; 4],
);

// Default
impl Default for Matrix4 {
    fn default() -> Self {
        Self::identity()
    }
}

// Matrix
impl Matrix for Matrix4 {
    fn zero() -> Self {
        // TODO: mactro-ize
        Self(
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
        )
    }

    fn identity() -> Self {
        // TODO: mactro-ize
        Self(
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        )
    }

    fn size() -> usize {
        4
    }

    fn get(&self, i: usize, j: usize) -> f32 {
        match i {
            0 => self.0[j],
            1 => self.1[j],
            2 => self.2[j],
            3 => self.3[j],
            _ => panic!(),
        }
    }

    fn set(&mut self, i: usize, j: usize, value: f32) {
        match i {
            0 => self.0[j] = value,
            1 => self.1[j] = value,
            2 => self.2[j] = value,
            3 => self.3[j] = value,
            _ => panic!(),
        }
    }

    fn row(&self, j: usize) -> &[f32] {
        match j {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!(),
        }
    }

    fn column(&self, _i: usize) -> &[f32] {
        todo!()
    }

    fn transpose(&mut self) {
        let size = self::Matrix4::size();
        for i in 0..size {
            for j in i..size {
                let aux = self.get(i, j);

                self.set(i, j, self.get(j, i));
                self.set(j, i, aux);
            }
        }
    }
}

// Arithmetic
impl Mul<&Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: &Matrix4) -> Self::Output {
        let size = Matrix4::size();

        let a = self;
        let b = rhs;

        let mut mat = Matrix4::zero();

        for i in 0..size {
            for j in 0..size {
                let mut s = 0.0;
                for k in 0..size {
                    s += a.get(i, k) * b.get(k, j);
                }
                mat.set(i, j, s);
            }
        }

        mat
    }
}

// Display
impl Display for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = Self::size();
        for j in 0..size {
            writeln!(f, "{:?}", self.row(j))?;
        }
        Ok(())
    }
}

// Methods
impl Matrix4 {
    pub fn translate(translation: Vector3) -> Matrix4 {
        let mut m = Matrix4::identity();
        m.set(4, 1, translation.x);
        m.set(4, 2, translation.y);
        m.set(4, 3, translation.z);
        m
    }

    #[rustfmt::skip]
    pub fn look_at(up: Vector3, right: Vector3, forward: Vector3, position: Vector3) -> Matrix4 {
        let u = up;
        let r = right;
        let f = forward;

        let t = Vector3 {
            x: -r.dot(position),
            y: -u.dot(position),
            z: -f.dot(position),
        };

        Self(
           [ r.x,  u.x,  f.x, 0.],
           [ r.y,  u.y,  f.y, 0.],
           [ r.z,  u.z,  f.z, 0.],
           [-t.x, -t.y, -t.z, 1.],
       )
    }
}
