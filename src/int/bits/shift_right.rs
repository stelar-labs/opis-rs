use crate::Int;
use std::ops::{Shr, ShrAssign};

impl Shr<usize> for Int {

    type Output = Self;

    fn shr(self, shifts: usize) -> Self::Output {
        shift_right(&self, shifts)
    }

}

impl Shr<usize> for &Int {

    type Output = Int;

    fn shr(self, shifts: usize) -> Int {
        shift_right(self, shifts)
    }

}

impl ShrAssign<usize> for Int {

    fn shr_assign(&mut self, shifts: usize) {
        *self = shift_right(self, shifts)
    }

}

fn shift_right(a: &Int, shifts: usize) -> Int {

    let mut bits = a.bits.clone();

    if shifts <= bits.len() - 1 {

        for _ in 0..shifts {
            bits.pop();
        }
        
        Int { bits }

    } else {
        Int::zero()
    }

}
