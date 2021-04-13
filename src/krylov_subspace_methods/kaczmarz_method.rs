use crate::krylov_subspace_methods::krylov_essentials::Krylov;
use std::path::Path;
use crate::common::matrix::matrix::{Vector, Matrix};
use crate::operations::matrix_operations::MatrixOperations;

pub struct Kaczmarz;

impl Krylov<f64> for Kaczmarz {
    fn new_from_path(_path: &Path) -> Vector<f64> {
        unimplemented!()
    }

    fn new_from_matrix(a: Matrix<f64>, b: Vector<f64>, _eps: f64) -> Matrix<f64> {

        let mut x_old: Vector<f64> = Vector::new(
            vec![0f64; b.nrows()],
            1,
            b.nrows()
        );

        let mut x_new: Vector<f64>;
        let mut temp: Vec<f64>;
        let mut i: usize;

        for k in 0..200 {
            i = k % a.nrows();
            temp = vec![0f64; a.nrows()];
            temp.copy_from_slice(&a.data()[i * a.ncols()..i * a.ncols() + a.nrows()]);

            let ai: Vector<f64> = Vector::new(
                temp,
                1,
                b.nrows()

            );

            let scalar: f64 = (b.data()[i] - MatrixOperations::dot(&ai, &x_old).unwrap()) /
                MatrixOperations::dot(&ai, &ai).unwrap();

            let norm_vec: Vector<f64> = MatrixOperations::add(&ai, &ai, scalar, 0f64);


            x_new = MatrixOperations::add(&x_old, &norm_vec, 1f64, 1f64);


            let first = MatrixOperations::mul(&a, &x_new).unwrap();

            let residuum = MatrixOperations::euclidean_norm(
                &MatrixOperations::add(
                    &first, &b, 1f64, -1f64));

            x_old = x_new.to_owned();

        }

        x_old
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

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
            1,
            3,
        );

        let test: Vector<f64> = Kaczmarz::new_from_matrix(a, b, 0.001);

        assert_approx_eq!(test.data()[0], 5f64, 0.001);
        assert_approx_eq!(test.data()[1], 3f64, 0.001);
        assert_approx_eq!(test.data()[2], -2f64, 0.001);
    }
}