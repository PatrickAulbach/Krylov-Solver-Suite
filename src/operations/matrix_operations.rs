use crate::common::matrix::matrix::{Vector, Matrix};
use num::Num;
use std::ops::{Mul, Add, AddAssign, Neg};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::common::matrix::square_root::Sqrt;
use crate::common::matrix::dimensions::{DimensionError, Dimensions};

pub struct MatrixOperations<T> {
    phantom_data: PhantomData<T>
}

impl<T> MatrixOperations<T> where T: Mul<Output=T> + Add<Output=T>, T: FromStr + Copy + Num + AddAssign {
    pub fn add(a: Matrix<T>, b: Matrix<T>, alpha: T, beta: T) -> Matrix<T> {
        if alpha == T::zero() && beta == T::one() {
            b
        } else if beta == T::zero() && alpha == T::one() {
            a
        } else {
            let test = a.nrows() * a.ncols();
            let mut data: Vec<T> = Vec::new();
            for i in 0..a.data().len() {
                data.push(a.data()[i] * alpha + b.data()[i] * beta);
            }
            Matrix::new(data, a.nrows(), b.nrows())
        }
    }

    pub fn euclidean_norm(vector: Vector<T>) -> T where T: Sqrt {
        let mut norm = T::zero();

        for i in 0..vector.data().len() {
            norm += vector.data()[i] * vector.data()[i];
        }

        norm.sqrt()
    }

    pub fn mul(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>, DimensionError> {
        type Error = DimensionError;
        if a.nrows() != b.ncols() {
            return Err(DimensionError::InvalidDimension);
        }

        let mut data: Vec<T> = Vec::new();
        let mut buf: T = T::zero();

        for k in 0..a.nrows() {
            for i in 0..a.data().len() {
                unimplemented!()
            }
        }

        let data = data;

        //Ok(Matrix::new(&data, 0, 0));
        unimplemented!()
    }

    pub fn dot(a: Vector<T>, b: Vector<T>) -> Result<T, DimensionError> {
        if a.nrows() != 1 ||
            b.nrows() != 1 ||
            a.ncols() != b.ncols() {
            return Err(DimensionError::InvalidDimension);
        }

        let mut dot = T::zero();

        for i in 0..a.ncols() {
            dot += a.data()[i] * b.data()[i];
        }

        let dot = dot;

        //Ok(dot);
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;


    #[test]
    fn add_with_alpha_zero_should_return_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );

        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 1f64);

        assert_eq!(added_matrix.data(), &vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn add_with_alpha_zero_and_beta_greater_one_should_return_beta_times_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0
            ],
            3,
            3,
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 2f64);

        assert_eq!(added_matrix.data(), &vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0]);
    }

    #[test]
    fn euclidian_norm_should_be_correct() {
        let vector: Vector<f64> = Vector::new(
            vec![
                1.0, 2.0, 3.0
            ],
            3,
            1,
        );

        let norm = MatrixOperations::euclidean_norm(vector);

        assert_approx_eq!(3.74165738677, norm, 1e-3f64);
    }

    #[ignore]
    #[test]
    fn test_matrix_matrix_multiplication_with_3x3_matrices() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0
            ],
            3,
            3
        );

        let added_matrix: Matrix<f64> = MatrixOperations::mul(first_matrix, second_matrix).unwrap();

        assert_eq!(added_matrix.data(), &vec![12.0, 12.0, 12.0, 15.0, 15.0, 15.0, 18.0, 18.0, 18.0]);
    }
    #[ignore]
    #[test]
    fn test_dot_product_with_vectors() {
        let a: Vector<f64> = Vector::new(
            vec![1f64, 2f64, 3f64, 4f64, 5f64],
            1,
            5
        );
        let b: Vector<f64> = Vector::new(
            vec![1f64, 2f64, 3f64, 4f64, 5f64],
            1,
            5
        );

        let dot: f64 = MatrixOperations::dot(a, b).unwrap();

        assert_eq!(dot, 15f64);
    }
}
