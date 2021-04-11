use crate::common::matrix::matrix::{Vector, Matrix};
use num::Num;
use std::ops::{Mul, Add, AddAssign};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::common::matrix::square_root::Sqrt;
use crate::common::matrix::dimensions::DimensionError;

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
            let mut data: Vec<T> = Vec::new();
            for i in 0..a.data().len() {
                data.push(a.data()[i] * alpha + b.data()[i] * beta);
            }
            Matrix::new(data, a.ncols(), a.nrows())
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

        if b.nrows() == 1 {
            Ok(Matrix::new(MatrixOperations::matrix_vector_mul(&a, &b), a.ncols(), b.nrows()))
        } else {
            Ok(Matrix::new(MatrixOperations::matrix_matrix_mul(&a, &b), a.ncols(), b.nrows()))
        }

    }

    pub fn dot(a: Matrix<T>, b: Matrix<T>) -> Result<T, DimensionError> {
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

        Ok(dot)
    }

    fn matrix_vector_mul(a: &Matrix<T>, b: &Matrix<T>) -> Vec<T>{
        let mut data: Vec<T> = Vec::new();
        let mut buf: T;

        for i in 0..a.nrows() {
            buf = T::zero();
            for j in 0..b.ncols() {
                buf += a.data()[i * a.ncols() + j] * b.data()[j];
            }
            data.push(buf);
        }

        let data = data;
        data
    }

    fn matrix_matrix_mul(a: &Matrix<T>, b: &Matrix<T>) -> Vec<T> {
        let mut data: Vec<T> = Vec::new();
        let mut buf: T;

        for i in 0..a.nrows() {
            for j in 0..b.ncols() {
                buf = T::zero();
                for k in 0..a.ncols() {
                    buf += a.data()[i * a.ncols() + k] * b.data()[k * b.ncols() + j];
                }
                data.push(buf);
            }
        }

        let data = data;
        data
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
    fn add_with_beta_negative_should_return_correct_solution() {
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

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 1f64, -1f64);

        assert_eq!(added_matrix.data(), &vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn add_vector_with_beta_negative_should_return_correct_solution() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0
            ],
            3,
            1,
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0
            ],
            3,
            1,
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 1f64, -1f64);

        assert_eq!(added_matrix.data(), &vec![0.0, 0.0, 0.0]);
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

        let multiplicated_matrix: Matrix<f64> = MatrixOperations::mul(first_matrix, second_matrix).unwrap();

        assert_eq!(multiplicated_matrix.data(), &vec![12.0, 15.0, 18.0, 12.0, 15.0, 18.0, 12.0, 15.0, 18.0]);
    }

    #[test]
    fn test_matrix_matrix_multiplication_with_non_symmetric_matrices() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            2
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 2.0,
                4.0, 5.0,
                7.0, 8.0
            ],
            2,
            3
        );

        let multiplicated_matrix: Matrix<f64> = MatrixOperations::mul(first_matrix, second_matrix).unwrap();

        assert_eq!(multiplicated_matrix.data(), &vec![12.0, 15.0, 12.0, 15.0]);
    }

    #[test]
    fn test_matrix_vector_multiplication() {
        let matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                0.0, 2.0, 5.0,
                2.0, 5.0, -1.0
            ],
            3,
            3
        );
        let vec: Vector<f64> = Vector::new(
            vec![
                2.0,
                2.0,
                2.0
            ],
            3,
            1
        );

        let matrix_vector: Matrix<f64> = MatrixOperations::mul(matrix, vec).unwrap();

        assert_eq!(matrix_vector.data(), &vec![6.0, 14.0, 12.0]);
    }

    #[test]
    fn test_dot_product_with_vectors() {
        let a: Vector<f64> = Vector::new(
            vec![1f64, 2f64, 3f64, 4f64, 5f64],
            5,
            1
        );
        let b: Vector<f64> = Vector::new(
            vec![1f64, 1f64, 1f64, 1f64, 1f64],
            5,
            1
        );

        let dot: f64 = MatrixOperations::dot(a, b).unwrap();

        assert_eq!(dot, 15f64);
    }
}
