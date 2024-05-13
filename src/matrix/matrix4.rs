use std::{fmt::Display, ops::Mul};

use crate::Matrix;
use crate::OutOfBoundsError;
use crate::Vector;
use crate::Vector3;

#[allow(unused)]
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

    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut f32> {
        match i {
            0 => Some(&mut self.0[j]),
            1 => Some(&mut self.1[j]),
            2 => Some(&mut self.2[j]),
            3 => Some(&mut self.3[j]),
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

    fn row(&self, j: usize) -> Vec<f32> {
        match j {
            0 => self.0.to_vec(),
            1 => self.1.to_vec(),
            2 => self.2.to_vec(),
            3 => self.3.to_vec(),
            _ => panic!(),
        }
    }

    fn column(&self, i: usize) -> Vec<f32> {
        vec![
            self.get(i, 0).unwrap(),
            self.get(i, 1).unwrap(),
            self.get(i, 2).unwrap(),
            self.get(i, 3).unwrap(),
        ]
    }

    fn transpose(&self) -> Self {
        let size = self::Matrix4::size();
        let mut mat = self.clone();
        for i in 0..size {
            for j in i..size {
                let aux = self.get(i, j);

                mat.set(i, j, mat.get(j, i).unwrap()).unwrap();
                mat.set(j, i, aux.unwrap()).unwrap();
            }
        }
        mat
    }
    
    fn inverse(&self) -> Self {
        fn _pivot(row: &[f32]) -> Option<f32> {
            for elem in row {
                if *elem != 0.0 {
                    return Some(*elem)
                }
            }
            None
        }

        fn _r_echelon(_mat: &Matrix4) -> Matrix4 {
            todo!()
        }
        
        fn _rr_echelon(_mat: &Matrix4) -> Matrix4 {
            todo!()
        }
        



        let matrix = self.clone();



        matrix
    }
    
    fn ptr(&self) -> *const f32 {
        &self.0[0]
    }
    
    fn rows(&self) -> Vec<Vec<f32>> {
        vec![
            self.row(0),
            self.row(1),
            self.row(2),
            self.row(3),
        ]
    }
    
    fn columns(&self) -> Vec<Vec<f32>> {
        vec![
            self.column(0),
            self.column(1),
            self.column(2),
            self.column(3),
        ]
    }
}

pub fn pivot_col_index(_col: &[f32]) -> usize {

    todo!()
}

pub fn is_row_echelon(mat: &Matrix4) -> bool {
    for col in mat.columns() {
        let mut zero_col = true;
        for elem in col {
            if zero_col == false && elem != 0.0 {
                return false;
            }
            if elem != 0.0 {
                zero_col = false;
            }
        }
    }

    true
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

impl Mul<Vector3> for &Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let size = Matrix4::size();

        let a = self;
        let b = [
            rhs.x,
            rhs.y,
            rhs.z,
            1.0,
        ];

        let mut out = [0.0; 4];

        for i in 0..size {
            let mut s = 0.0;
            for k in 0..size {
                s += a.get(i, k).unwrap() * b.get(k).unwrap();
            }
            out[i] = s;
        }

        Vector3 {
            x: out[0],
            y: out[1],
            z: out[2],
        }
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
    pub fn translate(translation: Vector3) -> Self {
        let mut m = Matrix4::identity();
        m.set(0, 3, translation.x).unwrap();
        m.set(1, 3, translation.y).unwrap();
        m.set(2, 3, translation.z).unwrap();
        m
    }

    pub fn uniform_scale(scale: f32) -> Self {
        let mut mat = Matrix4::identity();
        mat.set(0, 0, scale).unwrap();
        mat.set(1, 1, scale).unwrap();
        mat.set(2, 2, scale).unwrap();
        mat
    }

    pub fn scale(scale: Vector3) -> Self {
        let mut mat = Matrix4::identity();
        mat.set(0, 0, scale.x).unwrap();
        mat.set(1, 1, scale.y).unwrap();
        mat.set(2, 2, scale.z).unwrap();
        mat
    }

    #[rustfmt::skip]
    pub fn look_at(fwd: Vector3, up: Vector3, right: Vector3, eye: Vector3) -> Self {
        let (r, u, f, t) = (right, up, fwd, eye);
        Matrix4(
            [r.x, r.y, r.z, t.x],
            [u.x, u.y, u.z, t.y],
            [f.x, f.y, f.z, t.z],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    #[rustfmt::skip]
    pub fn look_at_dir(eye: Vector3, dir: Vector3, up: Vector3) -> Self {
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
        
        let a = f / aspect;
        let b = f;
        let c = (far + near) / (near - far);
        let d = (2.0 * far * near) / (near - far);
        let e = -1.0;
        
        Self(
            [a,   0.0, 0.0, 0.0],
            [0.0, b,   0.0, 0.0],
            [0.0, 0.0, c,   d  ],
            [0.0, 0.0, e,   0.0],
        )
    }

    #[rustfmt::skip]
    pub fn inverse_perspective(
        fovy_deg: f32,
        aspect: f32,
        near: f32,
        far: f32
    ) -> Self {
        let f = (fovy_deg.to_radians() / 2.0).tan().recip();
        
        let a = f / aspect;
        let b = f;
        let c = (far + near) / (near - far);
        let d = (2.0 * far * near) / (near - far);
        let e = -1.0;

        Self(
            [1.0 / a,   0.0,        0.0,        0.0],
            [0.0,       1.0 / b,    0.0,        0.0],
            [0.0,       0.0,        0.0,        1.0 / e],
            [0.0,       0.0,        1.0 / d,    (-c)/(d*e)],
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
