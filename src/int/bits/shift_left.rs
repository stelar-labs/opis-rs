use crate::{Int, Bit};
use std::ops::{Shl, ShlAssign};

impl Shl<usize> for Int {

    type Output = Self;

    fn shl(self, shifts: usize) -> Self::Output {
        shift_left(&self, shifts)
    }

}

impl Shl<usize> for &Int {

    type Output = Int;

    fn shl(self, shifts: usize) -> Int {
        shift_left(self, shifts)
    }
    
}

impl ShlAssign<usize> for Int {

    fn shl_assign(&mut self, shifts: usize) {
        *self = shift_left(self, shifts)
    }

}

fn shift_left(a: &Int, shifts: usize) -> Int {

    let mut bits = a.bits.clone();

    for _ in 0..shifts {
        bits.push(Bit::Zero)
    }

    Int { bits }

}
