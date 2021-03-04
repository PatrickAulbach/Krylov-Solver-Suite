use crate::common::matrix::matrix::Vector;
use num::Num;
use std::ops::{Mul, Div, Sub, Add, Neg};
use std::marker::PhantomData;

pub struct VectorOperations<T> { 
    _phantom_data: PhantomData<T>
}

impl<T: Num> VectorOperations<T> {
    pub fn addition(first_vec: Vector<T>, second_vec: Vector<T>, alpha: T, beta: T) -> Vector<T> {
        if alpha == T::zero() {
            second_vec
        } else if beta == T::zero() {
            first_vec
        } else {
            first_vec.add(second_vec, alpha, beta)
        }
    }
    /*
    pub fn euclidean_norm<'a>(vector: &Vector<T>) -> T where T: Num + std::ops::Mul<Output = &'a T> {
        let mut norm = T::zero();

        for i in 0..vector.ncols() {
            norm.into() += &vector.data().vec()[i].into() * &vector.data().vec()[i].into();
        }

        return norm;
    }

     */
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