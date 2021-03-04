use crate::common::matrix::dimensions::DimensionError;

pub struct Data<T> {
    data: Vec<Vec<T>>
}

impl<T> Data<T> {
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

    pub fn vec(&self) -> &Vec<T> {
        &self.data[0]
    }

    pub fn matrix(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn check_consistency(&self) -> Result<(), DimensionError> {

        type Error = DimensionError;
        for column in self.data.iter() {
            if column.len() != self.nrows() {
                return Err(DimensionError::InvalidDimension);
            }
        }
        Ok(())
    }
}
