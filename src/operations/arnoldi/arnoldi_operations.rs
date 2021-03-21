use crate::common::matrix::matrix::{Vector, Matrix};
use crate::operations::matrix_operations::MatrixOperations;
use std::ops::{Neg, Add, Mul, AddAssign};
use std::str::FromStr;
use std::marker::PhantomData;
use num::Num;

pub struct ArnoldiOperations<T> {
    phantom_data: PhantomData<T>
}

impl<'a, T> ArnoldiOperations<T> where T: Mul<Output = T> + Add<Output = T>, T: FromStr + Copy + Num + AddAssign {
    pub fn arnoldi_orthogonalization(mut v: Vector<T>, q: Matrix<T>, h: Matrix<T>, k: usize) -> Vector<T>
        where T: Neg<Output = T> {

        for i in 0..q.ncols() {
            let q_vec = Vector::new(vec![q.data()[i].clone()], q.nrows(), 1);
            v = MatrixOperations::add(v, q_vec, T::one(), -h.data()[i]);
        }

        let v = v; // remove mut
        v
    }
}