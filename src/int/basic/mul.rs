use crate::{Bit, Int};
use std::ops::Mul;

impl Mul for Int {

    type Output = Self;
    
    fn mul(self, b: Self) -> Self {
        mul(&self, &b)
    }

}

impl Mul for &Int {

    type Output = Int;
    
    fn mul(self, b: Self) -> Int {
        mul(self, b)
    }

}

fn mul(a: &Int, b: &Int) -> Int {

    let precision =

        if a.bits.len() > b.bits.len() {

            a.bits.len() * 2
        
        } else {

            b.bits.len() * 2
        
        };

    let mut result = Int::zero();

    for i in 0..precision {

        let bit = b.get_right_bit(i, precision).unwrap();

        if bit == Bit::One {

            let shifted = a << i;

            result += shifted

        }

    }

    while result.bits.len() > 2 && result.bits[0] == result.bits[1] {

        result.bits.remove(0);
        
    }

    result

}
