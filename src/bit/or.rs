use crate::Bit;
use std::ops::BitOr;

impl BitOr for Bit {

    type Output = Self;
    
    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            _ => Bit::One
        }
    }

}