pub use Angle::*;

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Deg(f32),
    Rad(f32),
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Angle::Rad(value)
    }
}

impl Into<f32> for Angle {
    fn into(self) -> f32 {
        match self {
            Angle::Deg(d) => d.to_radians(),
            Angle::Rad(r) => r,
        }
    }
}

impl Angle {
    pub fn rad(self) -> f32 {
        match self {
            Angle::Deg(d) => d.to_radians(),
            Angle::Rad(r) => r,
        }
    }

    pub fn deg(self) -> f32 {
        match self {
            Angle::Deg(d) => d,
            Angle::Rad(r) => r.to_degrees(),
        }
    }
}
