use crate::Bit;
use std::ops::BitAnd;

impl BitAnd for Bit {

    type Output = Self;
    
    fn bitand(self, b: Self) -> Self::Output {
        and(&self, &b)
    }

}

impl BitAnd for &Bit {

    type Output = Bit;

    fn bitand(self, b: Self) -> Bit {
        and(self, b)
    }
    
}

fn and(a: &Bit, b: &Bit) -> Bit {

    if a == &Bit::One && b == &Bit::One {
        Bit::One
    } else {
        Bit::Zero
    }

}
