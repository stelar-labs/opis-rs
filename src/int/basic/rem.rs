use crate::{Bit, Int};
use crate::int::basic::div::div;
use std::error::Error;
use std::ops::Rem;

impl Rem for Int {

    type Output = Result<Self, Box<dyn Error>>;
    
    fn rem(self, b: Self) -> Result<Self, Box<dyn Error>> {
        
        if self == Int::zero() {

            Ok(Int::zero())

        } else if b == Int::zero() {

            Err("a/0 is undefined!")?

        } else {
    
            let (_, remainder) = div(&self.positive(), &b.positive());
    
            if self.bits[0] == Bit::One {

                Ok(remainder.negative())

            } else {

                Ok(remainder)

            }
    
        }
    }

}

impl Rem for &Int {

    type Output = Result<Int, Box<dyn Error>>;
    
    fn rem(self, b: Self) -> Result<Int, Box<dyn Error>> {
        self.clone() % b.clone()
    }

}