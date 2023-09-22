use std::error::Error;

use crate::Matrix;

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<'_> {
    
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Matrix(value.iter().map(|x| x.iter().map(|y| y.try_into()).collect()).collect())
    }
}