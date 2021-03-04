use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::common::matrix::Data;
use num::Num;
use std::str::FromStr;
use std::fmt::Debug;

pub struct FileReader<T> {
    data: Data<T>
}

impl<T> FileReader<T> where T: Num + FromStr {
    pub fn read_matrix_from_file(&self, path: &Path) -> Data<T> where <T as FromStr>::Err: Debug {
        let mut file = BufReader::new(File::open(path).unwrap());

        let data = file.lines()
            .map(|f| f.unwrap().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
            .collect();

        self.data.new(data)
    }
}
