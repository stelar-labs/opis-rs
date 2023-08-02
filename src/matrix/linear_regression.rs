use std::error::Error;

use crate::Matrix;


impl<T> Matrix<T>
    where T: Clone
    + std::ops::Mul
    + std::ops::Add
    + std::ops::Sub
    + std::cmp::PartialOrd 
    + std::ops::Div<Output=Result<T, Box<dyn Error>>>
    + std::ops::Add<Output=T>
    + std::ops::Mul<Output=T>
    + std::ops::Sub<Output=T>
    + std::fmt::Debug {

    pub fn linear_regression(&self, variables: &Matrix<T>, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>> {
        let x_transpose = variables.transpose()?;
        let xtx = (&x_transpose * variables)?;
        let xtx_inv = xtx.inverse_div_err(neg_one, zero, one)?;
        let xty = (&x_transpose * self)?;
        let coefficients = xtx_inv * xty;
        coefficients
    }
}

#[cfg(test)]
mod tests {
    
    use crate::Fraction;
    use super::*;

    #[test]
    fn test_linear_regression() {

        let zero = Fraction::try_from("0/0").unwrap();
        let one = Fraction::try_from("1/1").unwrap();
        let two = Fraction::try_from("2/1").unwrap();
        let three = Fraction::try_from("3/1").unwrap();
        let four = Fraction::try_from("4/1").unwrap();
        let neg_one = Fraction::try_from("-1/1").unwrap();
        let neg_two = Fraction::try_from("-2/1").unwrap();

        let y = Matrix(vec![
            vec![zero.clone()],
            vec![zero.clone()],
            vec![one.clone()],
            vec![one.clone()],
            vec![three.clone()]
            ]);

        let x = Matrix(vec![
            vec![one.clone(), neg_two, four.clone()],
            vec![one.clone(), neg_one.clone(), one.clone()],
            vec![one.clone(), zero.clone(), zero.clone()],
            vec![one.clone(), one.clone(), one.clone()],
            vec![one.clone(), two, four],
        ]);

        let b = Matrix(vec![
            vec![Fraction::try_from("4/7").unwrap()],
            vec![Fraction::try_from("7/10").unwrap()],
            vec![Fraction::try_from("3/14").unwrap()],
        ]);

        assert_eq!(y.linear_regression(&x, neg_one, zero, one).unwrap(), b);

    }

}