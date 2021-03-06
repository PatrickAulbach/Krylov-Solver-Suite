use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::common::matrix::Data;
use num::Num;
use std::str::FromStr;
use std::fmt::Debug;

pub struct FileReader<T>;

impl<T> FileReader<T> where T: Num + FromStr {
    pub fn read_matrix_from_file(path: &Path) -> Data<T> where <T as FromStr>::Err: Debug {
        let mut file = BufReader::new(File::open(path).unwrap());

        let data: Vec<Vec<T>> = file.lines()
            .map(|f| f.unwrap().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
            .collect();

        Data::new(data)
    }
}
