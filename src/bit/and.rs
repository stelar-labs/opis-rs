use crate::Bit;
use std::ops::BitAnd;

impl BitAnd for Bit {

    type Output = Self;
    
    fn bitand(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bit::One, Bit::One) => Bit::One,
            _ => Bit::Zero
        }
    }

}