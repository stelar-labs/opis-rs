use crate::Int;
use std::ops::BitAnd;

impl BitAnd for Int {

    type Output = Self;
    
    fn bitand(self, b: Self) -> Self::Output {
        and(&self, &b)
    }

}

impl BitAnd for &Int {
    
    type Output = Int;
    
    fn bitand(self, b: Self) -> Int {
        and(self, b)
    }

}

fn and(a: &Int, b: &Int) -> Int {

    let precision =

        if a.bits.len() > b.bits.len() {

            a.bits.len()
        
        } else {

            b.bits.len()
        
        };

    let mut and_bits = Vec::new();

    for i in 0..precision {

        let a_bit = a.get_left_bit(i, precision).unwrap();

        let b_bit = b.get_left_bit(i, precision).unwrap();

        let and_bit = a_bit & b_bit;

        and_bits.push(and_bit);

    }

    Int { bits: and_bits }

}
