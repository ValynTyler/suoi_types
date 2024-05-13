pub mod matrix4;
pub use matrix4::*;

#[derive(Debug)]
pub struct OutOfBoundsError;

pub trait Matrix {
    fn size() -> usize;
    fn ptr(&self) -> *const f32;

    fn get(&self, i: usize, j: usize) -> Option<f32>;
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut f32>;

    fn set(&mut self, i: usize, j: usize, value: f32) -> Result<(), OutOfBoundsError>;

    fn row(&self, j: usize) -> &[f32; 4];
    fn column(&self, i: usize) -> Vec<f32>;
    
    fn rows(&self) -> Vec<Vec<f32>>;
    fn columns(&self) -> Vec<Vec<f32>>;

    fn swap_rows(&mut self, i1: usize, i2: usize);

    fn zero() -> Self;
    fn identity() -> Self;
    fn transpose(&self) -> Self;
    fn inverse(&self) -> Self;
}
