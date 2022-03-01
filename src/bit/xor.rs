use crate::Bit;
use std::ops::BitXor;

impl BitXor for Bit {

    type Output = Self;
    
    fn bitxor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            _ => Bit::Zero
        }
    }

}