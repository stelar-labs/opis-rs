use std::ops::BitAnd;

use crate::Bit;
use crate::Int;

impl BitAnd for Int {

    type Output = Self;
    
    fn bitand(mut self, mut b: Self) -> Self::Output {

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

            let and_bit: Bit =
                match (a_bit, b_bit) {
                    (Bit::One, Bit::One) => Bit::One,
                    _ => Bit::Zero
                };

            res.push(and_bit)

        }

        res.reverse();

        Int {bits: res}

    }

}

impl BitAnd for &Int {
    
    type Output = Int;
    
    fn bitand(self, b: Self) -> Int {
        self.clone() & b.clone()
    }

}