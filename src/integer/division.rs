
use crate::{Integer, Bit};
use std::error::Error;
use std::ops::{Div};
use super::divider;

impl Div for Integer {
    type Output = Result<Self, Box<dyn Error>>;
    fn div(self, b: Self) -> Result<Self, Box<dyn Error>> {
        &self / &b
    }
}

impl Div<&u8> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn div(self, b: &u8) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self / &b_int
    }
}

impl Div<&u16> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn div(self, b: &u16) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self / &b_int
    }
}

impl Div<&u32> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn div(self, b: &u32) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self / &b_int
    }
}

impl Div<&u64> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn div(self, b: &u64) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self / &b_int
    }
}

impl Div<&u128> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn div(self, b: &u128) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self / &b_int
    }
}

impl Div<&usize> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn div(self, b: &usize) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self / &b_int
    }
}

impl Div for &Integer {

    type Output = Result<Integer, Box<dyn Error>>;

    fn div(self, b: Self) -> Result<Integer, Box<dyn Error>> {

        if self == &Integer::zero() {

            Ok(Integer::zero())

        } else if b == &Integer::zero() {

            Err("a/0 is undefined!")?

        } else {

            let a_pos =
                if self.0[0] == Bit::Zero {
                    self.clone()
                } else {
                    self.negate()
                };

            let b_pos =
                if b.0[0] == Bit::Zero {
                    b.clone()
                } else {
                    b.negate()
                };

            let (quotient, _) = divider::run(&a_pos, &b_pos);

            match (self.0[0], b.0[0]) {
                (Bit::Zero, Bit::Zero) => Ok(quotient),
                (Bit::One, Bit::One) => Ok(quotient),
                (Bit::Zero, Bit::One) => Ok(quotient.negate()),
                (Bit::One, Bit::Zero) => Ok(quotient.negate())
            }

        }
    
    }

}
