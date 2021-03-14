use crate::common::matrix::matrix::{Vector, Matrix};
use num::Num;
use std::ops::{Mul, Add, AddAssign, Neg};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::common::matrix::square_root::Sqrt;

pub struct MatrixOperations<T> {
    phantom_data: PhantomData<T>
}

impl<T> MatrixOperations<T> where T: Mul<Output=T> + Add<Output=T>, T: FromStr + Copy + Num + AddAssign {
    //compute alpha * A + beta * B
    pub fn add(a: Matrix<T>, b: Matrix<T>, alpha: T, beta: T) -> Matrix<T> {
        if alpha == T::zero() && beta == T::one() {
            b
        } else if beta == T::zero() && alpha == T::one() {
            a
        } else {
            let mut data: Vec<Vec<T>> = Vec::new();
            for i in 0..a.ncols() {
                let mut column_vec: Vec<T> = Vec::new();
                for j in 0..a.nrows() {
                    column_vec.push(a.data()[i][j] * alpha + b.data()[i][j] * beta);
                }
                data.push(column_vec);
            }
            Matrix::new(data)
        }
    }

    pub fn euclidean_norm(vector: Vector<T>) -> T where T: Sqrt {
        let mut norm = T::zero();

        for i in 0..vector.nrows() {
            norm += vector.data()[0][i] * vector.data()[0][i];
        }

        norm.sqrt()
    }

    pub fn matrix_matrix_multiplication(matrix: Matrix<T>, vector: Matrix<T>) -> Matrix<T> {

        let mut data: Vec<Vec<T>> = Vec::new();

        for k in 0..vector.ncols() {
            let mut column_vec: Vec<T> = Vec::new();
            for i in 0..matrix.ncols() {
                for j in 0..matrix.nrows() {
                    column_vec.push(matrix.data()[i][j] * vector.data()[j][k]);
                }
            }
            data.push(column_vec);
        }

        let data = data;
        Matrix::new(data)
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

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 1f64);

        assert_eq!(added_matrix.data()[0], vec![1.0, 1.0, 1.0]);
        assert_eq!(added_matrix.data()[1], vec![1.0, 1.0, 1.0]);
        assert_eq!(added_matrix.data()[2], vec![1.0, 1.0, 1.0]);
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

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 2f64);

        assert_eq!(added_matrix.data()[0], vec![2.0, 4.0, 6.0]);
        assert_eq!(added_matrix.data()[1], vec![8.0, 10.0, 12.0]);
        assert_eq!(added_matrix.data()[2], vec![14.0, 16.0, 18.0]);
    }

    #[test]
    fn euclidian_norm_should_be_correct() {
        let vector: Vector<f64> = Vector::new(
            vec![
                vec![1.0, 2.0, 3.0]
            ]
        );

        let norm = MatrixOperations::euclidean_norm(vector);

        assert_approx_eq!(3.74165738677, norm, 1e-3f64);
    }
}
