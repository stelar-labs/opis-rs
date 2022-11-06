use crate::Bit;
use std::ops::BitXor;

impl BitXor for Bit {
    type Output = Self;
    fn bitxor(self, b: Self) -> Self::Output {
        xor(&self, &b)
    }
}

impl BitXor<&Bit> for Bit {
    type Output = Self;
    fn bitxor(self, b: &Bit) -> Self::Output {
        xor(&self, b)
    }
}

impl BitXor for &Bit {
    type Output = Bit;
    fn bitxor(self, b: Self) -> Bit {
        xor(self, b)
    }
}

impl BitXor<Bit> for &Bit {
    type Output = Bit;
    fn bitxor(self, b: Bit) -> Bit {
        xor(self, &b)
    }
}

fn xor(a: &Bit, b: &Bit) -> Bit {
    match (a, b) {      
        (Bit::Zero, Bit::One) => Bit::One,
        (Bit::One, Bit::Zero) => Bit::One,
        _ => Bit::Zero
    }
}
