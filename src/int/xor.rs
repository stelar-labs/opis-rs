use std::ops::BitXor;

use crate::Bit;
use crate::Int;

impl BitXor for Int {

    type Output = Self;
    
    fn bitxor(mut self, mut b: Self) -> Self::Output {
        
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

            let xor_bit: Bit =
                match (a_bit, b_bit) {
                    (Bit::Zero, Bit::One) => Bit::One,
                    (Bit::One, Bit::Zero) => Bit::One,
                    _ => Bit::Zero
                };

            res.push(xor_bit)

        }

        res.reverse();

        Int {bits: res}

    }

}

impl BitXor for &Int {

    type Output = Int;
    
    fn bitxor(self, b: Self) -> Int {
        self.clone() ^ b.clone()
    }
    
}