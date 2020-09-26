pub(crate) struct Matrix {
    columns: usize,
    rows: usize,
    data: Vec<Vec<f64>>
}

impl Matrix {
    pub(crate) fn new(columns: usize, rows: usize) -> Matrix {
        Matrix {
            columns: columns,
            rows: rows,
            data: vec![vec![0.0f64; rows]; columns]
        }
    }
}