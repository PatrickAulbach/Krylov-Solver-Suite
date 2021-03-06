use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::common::matrix::Data;
use num::Num;
use std::str::FromStr;
use std::fmt::Debug;

pub struct FileReader<T> {
    data_type: T
}

impl<T> FileReader<T> where T: Num + FromStr {
    pub fn read_matrix_from_file(path: &Path) -> Data<T> where <T as FromStr>::Err: Debug {
        let file = BufReader::new(File::open(path).unwrap());

        let data: Vec<Vec<T>> = file.lines()
            .map(|f| f.unwrap().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
            .collect();

        Data::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_struct_should_have_f64_as_data_type() {
        let data: Data<f32> = FileReader::read_matrix_from_file(Path::new("src/common/reader/test.txt"));

        assert_eq!(1f32, data.matrix()[0][0]);
    }

    #[test]
    fn data_struct_should_have_f32_as_data_type() {
        let data: Data<f32> = FileReader::read_matrix_from_file(Path::new("src/common/reader/test.txt"));

        assert_eq!(1f32, data.matrix()[0][0]);
    }
}

