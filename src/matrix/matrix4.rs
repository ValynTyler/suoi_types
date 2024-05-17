use std::{fmt::Display, ops::Mul};

use crate::Angle;
use crate::Deg;
use crate::Matrix;
use crate::Matrix3;
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

    fn row(&self, i: usize) -> &[f32; 4] {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!(),
        }
    }
    
    fn row_mut(&mut self, i: usize) -> &mut [f32; 4] {
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!(),
        }
    }

    fn column(&self, j: usize) -> Vec<f32> {
        vec![
            self.get(0, j).unwrap(),
            self.get(1, j).unwrap(),
            self.get(2, j).unwrap(),
            self.get(3, j).unwrap(),
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
    
    fn ptr(&self) -> *const f32 {
        &self.0[0]
    }
    
    fn rows(&self) -> Vec<Vec<f32>> {
        vec![
            self.row(0).to_vec(),
            self.row(1).to_vec(),
            self.row(2).to_vec(),
            self.row(3).to_vec(),
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
    
    fn swap_rows(&mut self, i1: usize, i2: usize) {
        let aux = self.row(i1).clone();
        
        match i1 {
            0 => self.0 = *self.row(i2),
            1 => self.1 = *self.row(i2),
            2 => self.2 = *self.row(i2),
            3 => self.3 = *self.row(i2),
            _ => panic!(),
        }

        match i2 {
            0 => self.0 = aux,
            1 => self.1 = aux,
            2 => self.2 = aux,
            3 => self.3 = aux,
            _ => panic!(),
        }
    }
    
    fn mul_row(&mut self, i: usize, coef: f32) {
        for elem in self.row_mut(i) {
            *elem *= coef;
        }
    }
    
    fn add_row_mul(&mut self, i_from: usize, i_to: usize, coef: f32) {
        let mut from_row = self.row(i_from).clone();
        let to_row = self.row_mut(i_to);

        for elem in &mut from_row {
            *elem *= coef;
        }

        for i in 0..Self::size() {
            to_row[i] += from_row[i];
        }
    }
    
    fn det(&self) -> f32 {
          self.row(0)[0] * self.cofactor(0, 0)
        - self.row(1)[0] * self.cofactor(1, 0)
        + self.row(2)[0] * self.cofactor(2, 0)
        - self.row(3)[0] * self.cofactor(3, 0)
    }
    
    fn inverse(&self) -> Self {
        self.adjugate() * (1.0 / self.det())
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

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = self.clone();

        for i in 0..4 {
            for j in 0..4 {
                mat.row_mut(i)[j] *= rhs
            }
        }

        mat
    }
}

// Display
impl Display for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = Self::size();
        if let Some(p) = f.precision() {
            for i in 0..size {
                write!(f, "[")?;
                for j in 0..size-1 {
                    write!(f, "{:.p$}, ", self.row(i)[j])?;
                }
                write!(f, "{:.p$}", self.row(i)[size-1])?;
                writeln!(f, "]")?;
            }
        } else {
            for i in 0..size {
                write!(f, "[")?;
                for j in 0..size-1 {
                    write!(f, "{:.}, ", self.row(i)[j])?;
                }
                write!(f, "{:.}", self.row(i)[size-1])?;
                writeln!(f, "]")?;
            }
        } 
        Ok(())
    }
}

// Methods
impl Matrix4 {
    pub fn minor(&self, i: usize, j: usize) -> Matrix3 {
        let mut mat = Matrix3(
            [0.0; 3],
            [0.0; 3],
            [0.0; 3],
        );

        for y in 0..3 {
            for x in 0..3 {
                let p = match y < i {
                    true => 0,
                    false => 1,
                };

                let q = match x < j {
                    true => 0,
                    false => 1,
                };

                mat.row_mut(y)[x] = self.row(y + p)[x + q]
            }
        }
        
        mat
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        // f32::powf(-1.0, i as f32 + j as f32) * self.minor(i, j).det()
        let coef = match (i + j) % 2 == 0 {
            true => 1.0,
            false => -1.0,
        };
        coef * self.minor(i, j).det()
    }

    pub fn cofactor_matrix(&self) -> Matrix4 {
        let mut mat = Matrix4::zero();

        for i in 0..4 {
            for j in 0..4 {
                mat.row_mut(i)[j] = self.cofactor(i, j);
            }
        }

        mat
    }

    pub fn adjugate(&self) -> Matrix4 {
        self.cofactor_matrix().transpose()
    }

    pub fn translate(translation: Vector3) -> Self {
        let mut m = Matrix4::identity();
        m.set(0, 3, translation.x).unwrap();
        m.set(1, 3, translation.y).unwrap();
        m.set(2, 3, translation.z).unwrap();
        m
    }

    pub fn rotate_y(angle: Deg) -> Self {
        let theta = angle.rad().0;
        Matrix4(
            [theta.cos(),   0.0, theta.sin(),   0.0],
            [0.0,           1.0, 0.0,           0.0],
            [-theta.sin(),  0.0, theta.cos(),   0.0],
            [0.0,           0.0, 0.0,           1.0],
        )
    }

    pub fn rotate_around(axis: Vector3, angle: Deg) -> Self {
        let r = axis;
        let theta = angle.rad().0;
        
        let sin = theta.sin();
        let cos = theta.cos();

        Matrix4(
            [cos + r.x*r.x*(1.0 - cos),        r.x*r.y*(1.0 - cos) - r.z*sin,     r.x*r.z*(1.0 - cos) + r.y*sin, 0.],
            [r.y*r.x*(1.0 - cos) + r.z*sin,    cos + r.y*r.y*(1.0 - cos),         r.y*r.z*(1.0 - cos) - r.x*sin, 0.],
            [r.z*r.x*(1.0 - cos) - r.y*sin,    r.z*r.y*(1.0 - cos) + r.x*sin,     cos + r.z*r.z*(1.0 - cos),     0.],
            [0.0, 0.0, 0.0, 1.0],
        )
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
