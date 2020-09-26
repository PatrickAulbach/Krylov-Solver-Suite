pub struct Matrix {
    columns: usize,
    rows: usize,
    data: Vec<Vec<f64>>
}

impl Matrix {
    pub(crate) fn new(columns: usize, rows: usize, data: Vec<Vec<f64>>) -> Self {
        Matrix {
            columns: columns,
            rows: rows,
            data: data
        }
    }

    pub fn get_matrix(&self) -> Vec<Vec<f64>> {
        self.data.to_vec()
    }

    pub fn get_column_len(&self) -> usize {
        self.columns
    }

    pub fn get_row_len(&self) -> usize {
        self.rows
    }

}