use crate::{Bit, Int};
use std::ops::{Sub, SubAssign};

impl Sub for Int {
    
    type Output = Self;
    
    fn sub(self, b: Self) -> Self {
        sub(&self, &b)
    }
}

impl Sub for &Int {

    type Output = Int;
    
    fn sub(self, b: Self) -> Int {
        sub(self, b)
    }

}

impl SubAssign for Int {
    
    fn sub_assign(&mut self, b: Self) {
        *self = sub(self, &b)
    }

}

fn sub(a: &Int, b: &Int) -> Int {

    if b.bits[0] == Bit::Zero {

        let b_neg = b.negative();

        a + &b_neg

    } else {

        a + b

    }
}
