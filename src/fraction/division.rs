use crate::Fraction;
use std::ops::Div;

impl Div for &Fraction {
    type Output = Result<Fraction, Box<dyn std::error::Error>>;
    fn div(self, b: Self) -> Self::Output {
        if b == &Fraction::zero() {
            Err("a/(0/1) is undefined!")?
        } else if self == &Fraction::zero() {
            Ok(Fraction::zero())
        } else {   
            Ok(self * &b.reciprocal().unwrap())
        }
    }
}


impl Div for Fraction {

    type Output = Result<Fraction, Box<dyn std::error::Error>>;

    fn div(self, b: Self) -> Self::Output {
        &self / &b
    }

}
