use std::{fmt::Display, ops::Mul};

use crate::Matrix;
use crate::OutOfBoundsError;
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

    fn get(&self, i: usize, j: usize) -> Option<f32> {
        match i {
            0 => Some(self.0[j]),
            1 => Some(self.1[j]),
            2 => Some(self.2[j]),
            3 => Some(self.3[j]),
            _ => None,
        }
    }

    fn set(&mut self, i: usize, j: usize, value: f32) -> Result<(), OutOfBoundsError> {
        match i {
            0 => self.0[j] = value,
            1 => self.1[j] = value,
            2 => self.2[j] = value,
            3 => self.3[j] = value,
            _ => return Err(OutOfBoundsError),
        };
        Ok(())
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

                self.set(i, j, self.get(j, i).unwrap()).unwrap();
                self.set(j, i, aux.unwrap()).unwrap();
            }
        }
    }
    
    fn transposition(&self) -> Self {
        let mut clone = self.clone();
        clone.transpose();
        clone
    }
    
    #[rustfmt::skip]
    fn ptr(&self) -> *const f32 {
        &self.0[0]
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
                    s += a.get(i, k).unwrap() * b.get(k, j).unwrap();
                }
                mat.set(i, j, s).unwrap();
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
        m.set(0, 3, translation.x).unwrap();
        m.set(1, 3, translation.y).unwrap();
        m.set(2, 3, translation.z).unwrap();
        m
    }

    #[rustfmt::skip]
    pub fn look_at(fwd: Vector3, up: Vector3, right: Vector3, eye: Vector3) -> Matrix4 {
        let (r, u, f, t) = (right, up, fwd, eye);
        Matrix4(
            [r.x, r.y, r.z, t.x],
            [u.x, u.y, u.z, t.y],
            [f.x, f.y, f.z, t.z],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    #[rustfmt::skip]
    pub fn look_at_dir(eye: Vector3, dir: Vector3, up: Vector3) -> Matrix4 {
        let f = dir.unit();
        let r = f.cross(up).unit();
        let u = r.cross(f);

        Matrix4(
            [ r.x.clone(), u.x.clone(), -f.x.clone(), 0.0],
            [ r.y.clone(), u.y.clone(), -f.y.clone(), 0.0],
            [ r.z.clone(), u.z.clone(), -f.z.clone(), 0.0],
            [-eye.dot(r), -eye.dot(u),   eye.dot(f),  1.0],
        )
        // .transposition()
    }

    #[rustfmt::skip]
    pub fn perspective(
        fovy_deg: f32,
        aspect: f32,
        near: f32,
        far: f32,

    ) -> Self {
        let f = (fovy_deg.to_radians() / 2.0).tan().recip();
        
        Self(
            [f / aspect, 0.0,  0.0,                         0.0                                 ],
            [0.0,        f,    0.0,                         0.0                                 ],
            [0.0,        0.0,  (far + near) / (near - far), (2.0 * far * near) / (near - far)   ],
            [0.0,        0.0, -1.0,                         0.0                                 ],
        )
    }

    #[rustfmt::skip]
    pub fn ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let (r, l) = (right, left);
        let (t, b) = (top, bottom);
        let (f, n) = (far, near);
        Self(
            [2./(r-l),  0.,         0.,        -(r+l) / (r-l)   ],
            [0.,        2./(t-b),   0.,        -(t+b) / (t-b)   ],
            [0.,        0.,        -2./(f-n),  -(f+n) / (f-n)   ],
            [0.,        0.,         0.,         1.              ],
        )
    }
}
