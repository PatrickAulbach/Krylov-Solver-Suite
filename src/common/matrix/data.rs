use std::error::Error;
use std::fmt::{Debug, Formatter, Display, Result};
use crate::matrix::dimensions::DimensionError;

pub struct Data<T> {
    data: Vec<Vec<T>>
}

impl Data<T> {
    pub fn new(&self, data: Vec<Vec<T>>) -> Self {
        self.check_consistency().unwrap();
        Data {
            data
        }
    }

    pub fn ncols(&self) -> usize {
        self.data.len()
    }

    pub fn nrows(&self) -> usize {
        self.data[0].len()
    }

    pub fn check_consistency(&self) -> Result {

        type DimensionError = dyn Error;
        for column in &self.data.iter() {
            if column != self.nrows() {
                return Err(DimensionError::InvalidDimension);
            }
        }
        Ok(())
    }
}
