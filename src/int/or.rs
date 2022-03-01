use std::ops::BitOr;

use crate::Bit;
use crate::Int;

impl BitOr for Int {

    type Output = Self;
    
    fn bitor(mut self, mut other: Self) -> Self::Output {
         
        let mut res: Vec<Bit> = Vec::new();

        while !self.magnitude.is_empty() || !other.magnitude.is_empty() {

            let self_bit: Bit = match self.magnitude.pop() { Some(r) => r, None => Bit::Zero };

            let other_bit: Bit = match other.magnitude.pop() { Some(r) => r, None => Bit::Zero };

            let or_bit = self_bit | other_bit;

            res.push(or_bit)

        }

        res.reverse();

        Int {
            magnitude: res,
            sign: self.sign | other.sign
        }

    }

}

impl BitOr for &Int {
    
    type Output = Int;
    
    fn bitor(self, b: Self) -> Int {
        self.clone() | b.clone()
    }

}