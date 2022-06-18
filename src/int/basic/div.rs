use crate::{Bit, Int};
use std::error::Error;
use std::ops::Div;

impl Div for Int {

    type Output = Result<Self, Box<dyn Error>>;
    
    fn div(self, b: Self) -> Result<Self, Box<dyn Error>> {
        
        if self == Int::zero() {

            Ok(Int::zero())

        } else if b == Int::zero() {

            Err("a/0 is undefined!")?

        } else {

            let (quotient, _) = div(&self.positive(), &b.positive());

            match (self.bits[0], b.bits[0]) {

                (Bit::Zero, Bit::Zero) => Ok(quotient),
    
                (Bit::One, Bit::One) => Ok(quotient),
    
                (Bit::Zero, Bit::One) => Ok(quotient.negative()),
    
                (Bit::One, Bit::Zero) => Ok(quotient.negative())
    
            }

        }

    }

}

impl Div for &Int {

    type Output = Result<Int, Box<dyn Error>>;
    
    fn div(self, b: Self) -> Result<Int, Box<dyn Error>> {
        self.clone() / b.clone()
    }

}

pub fn div(numerator: &Int, denominator: &Int) -> (Int, Int) {

    let mut quotient = Int::zero();

    let mut remainder = Int { bits: numerator.bits[..1].to_vec() };

    numerator
        .bits
        .iter()
        .skip(1)
        .for_each(|x| {

            remainder.bits.push(*x);

            if &remainder > denominator {

                quotient.bits.push(Bit::One);
                
                remainder = &remainder - denominator;
                
            } else if &remainder == denominator {

                quotient.bits.push(Bit::One);

                remainder = Int::zero();

            } else {

                quotient.bits.push(Bit::Zero);

            };

        });

    while quotient.bits.len() > 2 && quotient.bits[0] == quotient.bits[1] {

        quotient.bits.remove(0);

    };

    (quotient, remainder)

}
