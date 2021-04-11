use crate::krylov_subspace_methods::krylov_essentials::Krylov;
use std::path::Path;
use crate::common::matrix::matrix::{Vector, Matrix};
use crate::operations::matrix_operations::MatrixOperations;

pub struct Kaczmarz;

impl Krylov<f64> for Kaczmarz {
    fn new_from_path(_path: &Path) -> Vector<f64> {
        unimplemented!()
    }

    fn new_from_matrix(a: Matrix<f64>, b: Vector<f64>, eps: f64) -> Matrix<f64> {

        let mut x_old: Vector<f64> = Vector::new(
            vec![0f64; b.ncols()],
            b.ncols(),
            1
        );

        let mut x_new: Vector<f64>;
        let mut temp: Vec<f64> = vec![0f64; 3];
        let mut i: usize;

        for k in 0..200 {
            i = k % 3;
            temp = vec![0f64; 3];
            temp.copy_from_slice(&a.data()[i*3..i*3 + 3]);

            let temp = temp;

            let ai: Vector<f64> = Vector::new(
                temp,
                b.ncols(),
                1

            );

            let scalar: f64 = (b.data()[i] - MatrixOperations::dot(ai.clone(), x_old.clone()).unwrap()) /
                MatrixOperations::dot(ai.clone(), ai.clone()).unwrap();
            let norm_vec: Vector<f64> = MatrixOperations::add(ai.clone(), ai.clone(), scalar, 0f64);

            x_new = MatrixOperations::add(x_old.clone(), norm_vec, 1f64, 1f64);

            let residuum = MatrixOperations::euclidean_norm(
                MatrixOperations::add(
                    MatrixOperations::mul(a.clone(), x_new.clone()).unwrap(), b.clone(), 1f64, -1f64));

            dbg!(residuum);


            x_old = x_new.to_owned();

        }

        x_old
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

        let a: Matrix<f64> =  Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                0.0, 2.0, 5.0,
                2.0, 5.0, -1.0
            ],
            3,
            3,
        );

        let b: Vector<f64> = Vector::new(
            vec![
                6.0, -4.0, 27.0
            ],
            3,
            1,
        );

        let test: Vector<f64> = Kaczmarz::new_from_matrix(a, b, 0.001);

        assert_eq!(test.data(), &vec![5f64, 3f64, -2f64]);
    }
}