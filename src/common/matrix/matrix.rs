use num::Num;
use std::ops::{Mul, Add};
use crate::common::matrix::dimensions::DimensionError;

pub type Vector<'a, T> = Matrix<'a, T>;

#[derive(Clone)]
pub struct Matrix<'a, T> {
    data: &'a Vec<T>,
    ncols: usize,
    nrows: usize,
    rhs: Option<&'a Vec<T>>,
}

impl<'a, T: Num> Matrix<'a, T> where T: Mul<Output = T> + Add<Output = T> {
    pub fn new(data: &'a Vec<T>, ncols: usize, nrows: usize) -> Self {
        Matrix {
            data,
            ncols,
            nrows,
            rhs: None
        }
    }

    pub fn check_consistency(&self) -> Result<(), DimensionError> {

        type Error = DimensionError;
        if self.ncols * self.nrows != self.data.len() {
            return Err(DimensionError::InvalidDimension);
        }

        Ok(())
    }

    pub fn ncols(&self) -> usize {
        self.ncols()
    }

    pub fn nrows(&self) -> usize {
        self.nrows()
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

}
