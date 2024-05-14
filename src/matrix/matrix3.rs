use std::fmt::Display;

pub struct Matrix3(
    pub(crate) [f32; 3],
    pub(crate) [f32; 3],
    pub(crate) [f32; 3],
);

impl Display for Matrix3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = 3;
        for j in 0..size {
            writeln!(f, "{:?}", self.row(j))?;
        }
        Ok(())
    }
}

impl Matrix3 {
    pub fn det(&self) -> f32 {
        let a = self.row(0)[0];
        let b = self.row(0)[1];
        let c = self.row(0)[2];

        let d = self.row(1)[0];
        let e = self.row(1)[1];
        let f = self.row(1)[2];
        
        let g = self.row(2)[0];
        let h = self.row(2)[1];
        let i = self.row(2)[2];

        a * e * i + b * f * g + c * d * h - c * e * g - b * d * i - a * f * h
    }

    pub fn row(&self, i: usize) -> &[f32; 3] {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(),
        }
    }

    pub fn row_mut(&mut self, i: usize) -> &mut [f32; 3] {
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!(),
        }
    }
}
