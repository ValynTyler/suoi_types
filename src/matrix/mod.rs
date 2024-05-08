pub mod matrix4;
pub use matrix4::*;

#[derive(Debug)]
pub struct OutOfBoundsError;

pub trait Matrix {
    fn size() -> usize;
    fn ptr(&self) -> *const f32;

    fn get(&self, i: usize, j: usize) -> Option<f32>;
    fn set(&mut self, i: usize, j: usize, value: f32) -> Result<(), OutOfBoundsError>;
    
    fn row(&self, j: usize) -> &[f32];
    fn column(&self, i: usize) -> &[f32];
    
    fn zero() -> Self;
    fn identity() -> Self;

    fn transpose(&mut self);
    fn transposition(&self) -> Self;
}
