use crate::common::matrix::matrix::{Matrix, Vector};
use num::Num;
use std::str::FromStr;

pub fn matrix_vector_multiplication<'a, T>(matrix: &Matrix<T>, vector: &Vector<T>) -> Vector<T> where T: Num + FromStr + Copy {

    unimplemented!()
}