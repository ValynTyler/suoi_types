use std::fmt::Display;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)?;
        Ok(())
    }
}
