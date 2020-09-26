use std::env;
use std::path::Path;

#[path = "vector_operations/vector_operations.rs"] mod vector_operations;
#[path = "matrix_vector_operations/matrix_vector_operations.rs"] mod matrix_vector_operations;
#[path = "matrix/matrix.rs"] mod matrix;

fn main() {

    // TODO: Matrix Object, because q vectors need to be saved
    //compute first q vector: ||b|| * b
    //n: Dimension of Krylov Subspace. Must be > 1
    //for k in 0..n
        //qk = A * qk-1

        //for j in 0..k-1
            //hj,k-1 = qj*qk
            //qk = qk - hj,k-1*qj
        //hk,k-1 = ||qk||
        //qk = qk * 1/hk,k-1

}
