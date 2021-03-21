use num::Num;
use std::ops::{Mul, Add};
use crate::common::matrix::dimensions::DimensionError;

pub type Vector<T> = Matrix<T>;

#[derive(Clone)]
pub struct Matrix<T> {
    data: Vec<T>,
    ncols: usize,
    nrows: usize,
    rhs: Option<Vec<T>>,
}

impl<T: Num> Matrix<T> where T: Mul<Output = T> + Add<Output = T> {
    pub fn new(data: Vec<T>, ncols: usize, nrows: usize) -> Self {
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
