pub(crate) mod arnoldi_method {
    use std::path::Path;
    use crate::matrix::Matrix;
    use crate::matrix_vector_operations::matrix_vector_operations;
    use crate::vector_operations::vector_operations;
    use std::borrow::Borrow;

    //for now right hand side is an argument. TODO: Matrix object should be a general matrix object
    pub fn arnoldi_method(path: &Path, right_hand_side: &Vec<f64>) {
        //n: Dimension of Krylov Subspace. Must be > 1
        let n = 3;
        let mut qk: Vec<f64> = Vec::new();
        let matrix_vector_struct: Matrix = matrix_vector_operations::read_matrix_from_file(path);
        //Q holds all base vectors
        let mut Q = vec![vec![0.0f64; matrix_vector_struct.get_row_len()]; n];
        //empty Hessenberg matrix
        let mut H = vec![vec![0.0f64; n]; n];

        //compute first vector
        Q[0] = vector_operations::scalar_vector_multiplication((1.0 / vector_operations::euclidean_norm(right_hand_side)), right_hand_side);

        for i in 0..Q[0].len() {
            println!("{}", Q[0][i]);
        }
        println!();

        for k in 1..n {
            //qk = A * qk-1
            matrix_vector_operations::matrix_vector_multiplication(&matrix_vector_struct, &Q[k-1], &mut qk);

            for i in 0..qk.len() {
                println!("{}", qk[i]);
            }
            println!();

            //Gram Schmidt (Maybe as a method to keep arnoldi clean?)
            //gram_schmidt(&Q, k)
            vector_operations::gram_schmidt(&mut Q, &mut H, k);
            //hk,k-1 = ||qk||

            //qk = qk * 1/hk,k-1

        }
    }
}