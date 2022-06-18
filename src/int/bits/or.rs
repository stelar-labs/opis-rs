use crate::Int;
use std::ops::BitOr;

impl BitOr for Int {

    type Output = Self;
    
    fn bitor(self, b: Self) -> Self::Output {
        or(&self, &b)
    }

}

impl BitOr for &Int {
    
    type Output = Int;
    
    fn bitor(self, b: Self) -> Int {
        or(self, b)
    }

}

fn or(a: &Int, b: &Int) -> Int {

    let precision =

        if a.bits.len() > b.bits.len() {

            a.bits.len()
        
        } else {

            b.bits.len()
        
        };

    let mut or_bits = Vec::new();

    for i in 0..precision {

        let a_bit = a.get_left_bit(i, precision).unwrap();

        let b_bit = b.get_left_bit(i, precision).unwrap();

        let or_bit = a_bit | b_bit;

        or_bits.push(or_bit);

    }

    Int { bits: or_bits }

}
