use std::fmt::Display;

use crate::Matrix;

#[allow(unused)]
#[rustfmt::skip]
pub struct Matrix4(
    [f32; 4],
    [f32; 4],
    [f32; 4],
    [f32; 4],
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
            [1., 0., 0., 0.,],
            [0., 1., 0., 0.,],
            [0., 0., 1., 1.,],
            [0., 0., 0., 1.,],
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
}

// Display
impl Display for Matrix4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..Self::size() {
            write!(f, "[")?;
            for j in 0..Self::size() - 1 {
                write!(f, "{}, ", self.get(i, j))?;
            }
            write!(f, "{}]\n", self.get(i, Self::size() - 1))?;
        }
        Ok(())
    }
}
