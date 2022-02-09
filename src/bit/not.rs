use crate::Bit;
use std::ops::Not;

impl Not for Bit {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Bit::One => Bit::Zero,
            Bit::Zero => Bit::One
        }
    }
}

impl Not for &Bit {
    type Output = Bit;
    fn not(self) -> Bit {
        !self.clone()
    }
}