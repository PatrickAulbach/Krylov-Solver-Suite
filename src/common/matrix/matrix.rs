use num::Num;
use std::ops::{Mul, Add};
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

        if alpha == T::zero() && beta == T::one() {
            matrix
        } else if beta == T::zero() && alpha == T::one() {
            self
        } else {
            let mut data: Vec<Vec<T>> = Vec::new();
            for i in 0..self.ncols() {
                let mut column_vec: Vec<T> = Vec::new();
                for j in 0..self.nrows() {
                    column_vec.push(self.data[i][j] * alpha + matrix.data[i][j] * beta);
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
    use std::path::Path;

    #[test]
    fn add_with_alpha_zero_should_return_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0]
            ]
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0]
            ]
        );

        let added_matrix: Matrix<f64> = first_matrix.add(second_matrix, 0f64, 1f64);

        assert_eq!(added_matrix.data[0], vec![1.0, 1.0, 1.0]);
        assert_eq!(added_matrix.data[1], vec![1.0, 1.0, 1.0]);
        assert_eq!(added_matrix.data[2], vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn add_with_alpha_zero_and_beta_greater_one_should_return_beta_times_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0]
            ]
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0]
            ]
        );

        let added_matrix: Matrix<f64> = first_matrix.add(second_matrix, 0f64, 2f64);

        assert_eq!(added_matrix.data[0], vec![2.0, 4.0, 6.0]);
        assert_eq!(added_matrix.data[1], vec![8.0, 10.0, 12.0]);
        assert_eq!(added_matrix.data[2], vec![14.0, 16.0, 18.0]);
    }

}