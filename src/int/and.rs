use std::ops::BitAnd;

use crate::Bit;
use crate::Int;

impl BitAnd for Int {

    type Output = Self;
    
    fn bitand(mut self, mut other: Self) -> Self::Output {

        let mut res: Vec<Bit> = Vec::new();

        while !self.magnitude.is_empty() || !other.magnitude.is_empty() {

            let self_bit: Bit = match self.magnitude.pop() { Some(r) => r, None => Bit::Zero };

            let other_bit: Bit = match other.magnitude.pop() { Some(r) => r, None => Bit::Zero };

            let and_bit: Bit =
                match (self_bit, other_bit) {
                    (Bit::One, Bit::One) => Bit::One,
                    _ => Bit::Zero
                };

            res.push(and_bit)

        }

        res.reverse();

        Int {
            magnitude: res,
            sign: self.sign & other.sign
        }

    }

}

impl BitAnd for &Int {
    
    type Output = Int;
    
    fn bitand(self, b: Self) -> Int {
        self.clone() & b.clone()
    }

}