use std::ops::BitOr;

use crate::Bit;
use crate::Int;

impl BitOr for Int {

    type Output = Self;
    
    fn bitor(mut self, mut b: Self) -> Self::Output {
         
        let mut res: Vec<Bit> = Vec::new();

        while !self.bits.is_empty() || !b.bits.is_empty() {

            let a_bit: Bit =
                match self.bits.pop() {
                    Some(r) => r,
                    None => Bit::Zero
                };

            let b_bit: Bit =
                match b.bits.pop() {
                    Some(r) => r,
                    None => Bit::Zero
                };

            let or_bit =
                match (a_bit, b_bit) {
                    (Bit::Zero, Bit::Zero) => Bit::Zero,
                    _ => Bit::One
                };

            res.push(or_bit)

        }

        res.reverse();

        Int {bits: res}

    }

}

impl BitOr for &Int {
    
    type Output = Int;
    
    fn bitor(self, b: Self) -> Int {
        self.clone() | b.clone()
    }

}