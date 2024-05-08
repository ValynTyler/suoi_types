pub mod vector2;
pub use vector2::*;

pub mod vector3;
pub use vector3::*;

pub trait Vector {
    fn zero() -> Self;
    fn dot(&self, rhs: Self) -> f32;
    fn len(&self) -> f32;
    fn list(&self) -> Vec<f32>;
}
