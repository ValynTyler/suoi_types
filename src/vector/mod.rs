pub mod vector3;
pub use vector3::*;

pub trait Vector {
    fn len(&self) -> f32;
}
