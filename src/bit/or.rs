use crate::Bit;
use std::ops::BitOr;

impl BitOr for Bit {

    type Output = Self;
    
    fn bitor(self, b: Self) -> Self::Output {
        or(&self, &b)
    }

}

impl BitOr for &Bit {

    type Output = Bit;

    fn bitor(self, b: Self) -> Bit {
        or(self, b)
    }
    
}

fn or(a: &Bit, b: &Bit) -> Bit {

    if a == &Bit::Zero &&  b == &Bit::Zero {
        Bit::Zero
    } else {
        Bit::One
    }

}
