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
        if a.ncols() != b.nrows() {
            return Err(DimensionError::InvalidDimension);
        }

        if b.ncols() == 1 {
            Ok(Matrix::new(MatrixOperations::matrix_vector_mul(&a, &b), a.ncols(), b.nrows()))
        } else {
            Ok(Matrix::new(MatrixOperations::matrix_matrix_mul(&a, &b), a.ncols(), b.nrows()))
        }

    }

    pub fn dot(a: Matrix<T>, b: Matrix<T>) -> Result<T, DimensionError> {

        let mut dot = T::zero();

        for i in 0..a.nrows() {
            dot += a.data()[i] * b.data()[i];
        }

        let dot = dot;

        Ok(dot)
    }

    fn matrix_vector_mul(a: &Matrix<T>, b: &Matrix<T>) -> Vec<T>{
        let mut data: Vec<T> = Vec::new();
        let mut buf: T;

        for i in 0..a.ncols() {
            buf = T::zero();
            for j in 0..b.nrows() {
                buf += a.data()[i * a.nrows() + j] * b.data()[j];
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
