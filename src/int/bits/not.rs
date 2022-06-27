use crate::Int;
use std::ops::Not;

impl Not for Int {

    type Output = Self;
    
    fn not(self) -> Self::Output {
        not(&self)
    }

}

impl Not for &Int {

    type Output = Int;
    
    fn not(self) -> Int {
        not(self)
    }
    
}

fn not(a: &Int) -> Int {

    let not_bits =
    
        a.bits
            .iter()
            .map(|x| !x)
            .collect();

    Int { bits: not_bits }

}
