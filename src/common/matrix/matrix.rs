use num::Num;
use std::ops::{Mul, Add};
use std::collections::HashMap;
use crate::common::reader::file_reader::FileReader;
use std::path::Path;
use crate::common::matrix::dimensions::DimensionError;
use std::str::FromStr;
use std::fmt::Debug;

pub type Vector<T> = Matrix<T>;

pub struct Matrix<T: Num + FromStr + Copy> {
    data: Vec<Vec<T>>,
}

impl<T: Num> Matrix<T> where T: Mul<Output = T> + Add<Output = T>, T: FromStr + Copy {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Matrix {
             data
        }
    }

    //compute alpha * A + beta * B
    pub fn add(self, matrix: Matrix<T>, alpha: T, beta: T) -> Matrix<T> where <T as FromStr>::Err: Debug {

        if alpha == T::zero() {
            matrix
        } else if beta == T::zero() {
            self
        } else {
            let mut data: Vec<Vec<T>> = Vec::new();
            for i in 0..self.ncols() {
                let mut column_vec: Vec<T> = Vec::new();
                for j in 0..self.nrows() {
                    column_vec.push(self.data[i][j] + alpha + matrix.data[i][j] + beta);
                }
                data.push(column_vec);
            }
            Matrix::new(data)
        }
    }

    pub fn check_consistency(&self) -> Result<(), DimensionError> {

        type Error = DimensionError;
        for column in self.data.iter() {
            if column.len() != self.nrows() {
                return Err(DimensionError::InvalidDimension);
            }
        }
        Ok(())
    }

    pub fn ncols(&self) -> usize {
        self.data.len()
    }

    pub fn nrows(&self) -> usize {
        self.data[0].len()
    }

    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::reader::file_reader::FileReader;

    #[test]
    fn add_with_alpha_zero_should_return_b() {

    }

}