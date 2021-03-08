use crate::common::matrix::matrix::{Vector, Matrix};
use num::Num;
use std::ops::{Mul, Add};
use std::marker::PhantomData;
use std::str::FromStr;

pub struct MatrixOperations<T> {
    phantom_data: PhantomData<T>
}

impl<T> MatrixOperations<T> where T: Mul<Output = T> + Add<Output = T>, T: FromStr + Copy + Num {
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
                    column_vec.push(a.data[i][j] * alpha + b.data[i][j] * beta);
                }
                data.push(column_vec);
            }
            Matrix::new(data)
        }
    }
}

impl<'a, T: 'a> MatrixOperations<T> where &'a T: Mul<Output = &'a T> + Add<Output = &'a T> + FromStr + Copy + Num, T: std::ops::AddAssign + Num + Copy {

    pub fn euclidean_norm(vector: &Vector<T>) -> T {
        let mut norm = T::zero();

        for i in 0..vector.ncols() {
            norm += vector.data[i][0] * vector.data[i][0];
        }

        norm
    }
}

/*
pub(crate) mod vector_operations {




    pub fn gram_schmidt(base_vector_matrix: &mut Vec<Vec<f64>>, hessenberg_matrix: &mut Vec<Vec<f64>>, iteration_count: usize) {
        for i in 0..iteration_count {
            //hj,k-1 = qj*qk
            hessenberg_matrix[i][iteration_count - 1] = scalar_product(&base_vector_matrix[i], &base_vector_matrix[iteration_count]);
            //qk = qk - hj,k-1*qj
            let second_vector = scalar_vector_multiplication(hessenberg_matrix[i][iteration_count - 1], &mut base_vector_matrix[i]);

            vector_addition_subtraction(&mut base_vector_matrix[iteration_count],
                                        &second_vector,
                                        true);
        }
    }


}

 */