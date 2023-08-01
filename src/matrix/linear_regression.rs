use std::error::Error;

use crate::Matrix;


impl<T> Matrix<T>
    where T: Clone
    + std::ops::Mul
    + std::ops::Add
    + std::ops::Sub
    + std::cmp::PartialOrd 
    + std::ops::Div<Output=T>
    + std::ops::Add<Output=T>
    + std::ops::Mul<Output=T>
    + std::ops::Sub<Output=T>
    + std::fmt::Debug {

    pub fn linear_regression(&self, variables: &Matrix<T>, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>> {
        let x_transpose = variables.transpose()?;
        let xtx = (&x_transpose * variables)?;
        let xtx_inv = xtx.inverse(neg_one, zero, one)?;
        let xty = (&x_transpose * self)?;
        let coefficients = xtx_inv * xty;
        coefficients
    }
}