pub mod matrix4;
pub use matrix4::*;

pub trait Matrix {
    fn size() -> usize;

    fn get(&self, i: usize, j: usize) -> f32;
    fn set(&mut self, i: usize, j: usize, value: f32);
    
    fn row(&self, j: usize) -> &[f32];
    fn column(&self, i: usize) -> &[f32];
    
    fn zero() -> Self;
    fn identity() -> Self;

    fn transpose(&mut self);
    fn transposition(&self) -> Self;
}
