use std::fmt::{Debug, Display, Formatter, Result};
use std::error::Error;

pub enum Dimensions {
    VECTOR = 1
}

pub enum DimensionError {
    InvalidDimension,
    MatrixOpsError
}

impl Error for DimensionError {

}

impl DimensionError {
    pub fn message(&self) -> &str {
        match self {
            DimensionError::InvalidDimension => "Invalid Dimension",
            DimensionError::MatrixOpsError => ""
        }
    }
}

impl Debug for DimensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message())
    }
}

impl Display for DimensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message())
    }
}