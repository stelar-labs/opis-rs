use crate::{Fraction, Integer};
use std::ops::Div;

impl Div for &Fraction {

    type Output = Result<Fraction, Box<dyn std::error::Error>>;

    fn div(self, b: Self) -> Self::Output {

        if self.1 != Integer::zero() {

            if b.0 != Integer::zero() {

                if b.1 != Integer::zero() {

                    Ok(self * &b.reciprocal())

                } else {

                    Err("Denominator of B is zero!")?

                }

            } else {

                Err("Numerator of B is zero!")?

            }

        } else {

            Err("Denominator of A is zero!")?

        }

    }

}


impl Div for Fraction {

    type Output = Result<Fraction, Box<dyn std::error::Error>>;

    fn div(self, b: Self) -> Self::Output {

        &self / &b

    }

}