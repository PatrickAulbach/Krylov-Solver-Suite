use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::common::matrix::Data;

pub struct FileReader {
    data: Data<T>
}

impl FileReader {
    pub fn read_matrix_from_file(&self, path: &Path) -> Data<T> {
        let mut file = BufReader::new(File::open(path).unwrap());

        let data = file.lines()
            .map(|f| f.unwrap().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
            .collect();

        self.data.new(data)
    }
}
