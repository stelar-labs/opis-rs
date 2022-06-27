use crate::Int;
use std::ops::BitXor;

impl BitXor for Int {

    type Output = Self;
    
    fn bitxor(self, b: Self) -> Self::Output {
        xor(&self, &b)
    }

}

impl BitXor for &Int {

    type Output = Int;
    
    fn bitxor(self, b: Self) -> Int {
        xor(self, b)
    }
    
}

fn xor(a: &Int, b: &Int) -> Int {

    let precision =

        if a.bits.len() > b.bits.len() {

            a.bits.len()
        
        } else {

            b.bits.len()
        
        };

    let mut xor_bits = Vec::new();

    for i in 0..precision {

        let a_bit = a.get_left_bit(i, precision).unwrap();

        let b_bit = b.get_left_bit(i, precision).unwrap();

        let xor_bit = a_bit ^ b_bit;

        xor_bits.push(xor_bit);

    }

    Int { bits: xor_bits }

}
