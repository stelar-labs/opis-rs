use std::ops::Rem;

use crate::Bit;
use crate::Int;

use crate::int::div::divisor;

impl Rem for Int {

    type Output = Self;
    
    fn rem(self, b: Self) -> Self {
        
        if self == Int::zero() {
            Int::zero()
        }
        
        else if b == Int::zero() {
            panic!("a/0 is undefined!")
        }
        
        else {
    
            let (_, r) = divisor(self.bits[1..].to_vec(), b.bits[1..].to_vec());
    
            if self.bits[0] == Bit::One && r != vec![Bit::Zero] {
                Int {bits: [vec![Bit::One], r].concat()}
            } else {
                Int {bits: [vec![Bit::Zero], r].concat()}
            }
    
        }
    }

}

impl Rem for &Int {

    type Output = Int;
    
    fn rem(self, b: Self) -> Int {
        self.clone() % b.clone()
    }

}