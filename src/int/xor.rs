use std::ops::BitXor;

use crate::Bit;
use crate::Int;

impl BitXor for Int {

    type Output = Self;
    
    fn bitxor(mut self, mut other: Self) -> Self::Output {
        
        let mut res: Vec<Bit> = Vec::new();

        while !self.magnitude.is_empty() || !other.magnitude.is_empty() {

            let self_bit: Bit = match self.magnitude.pop() { Some(r) => r, None => Bit::Zero };

            let other_bit: Bit = match other.magnitude.pop() { Some(r) => r, None => Bit::Zero };

            let xor_bit: Bit = self_bit ^ other_bit;

            res.push(xor_bit)

        }

        res.reverse();

        Int { magnitude: res, sign: self.sign ^ other.sign }

    }

}

impl BitXor for &Int {

    type Output = Int;
    
    fn bitxor(self, b: Self) -> Int {
        self.clone() ^ b.clone()
    }
    
}