pub mod matrix4;
pub use matrix4::*;

pub trait Matrix {
    fn identity() -> Self;
    fn size() -> usize;
    fn get(&self, i: usize, j: usize) -> f32;
}
