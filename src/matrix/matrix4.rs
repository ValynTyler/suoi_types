use std::fmt::Display;

use nerd::vector::Vector3;

use crate::Matrix;

#[allow(unused)]
#[rustfmt::skip]
#[derive(Debug)]
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
    fn identity() -> Self {
        // TODO: mactro-ize
        Self(
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 1.],
            [0., 0., 0., 1.],
        )
    }

    fn size() -> usize {
        4
    }

    fn get(&self, i: usize, j: usize) -> f32 {
        match j {
            0 => self.0[i],
            1 => self.1[i],
            2 => self.2[i],
            3 => self.3[i],
            _ => panic!(),
        }
    }

    fn set(&mut self, i: usize, j: usize, value: f32) {
        match j {
            0 => self.0[i] = value,
            1 => self.1[i] = value,
            2 => self.2[i] = value,
            3 => self.3[i] = value,
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
            for j in 0..size {
                let p = self.get(i, j);
                let q = self.get(j, i);

                self.set(i, j, p);///
                self.set(j, i, q);
            }
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
