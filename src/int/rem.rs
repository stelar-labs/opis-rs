use std::ops::Rem;

use crate::Bit;
use crate::Int;

use crate::int::div::divisor;

impl Rem for Int {

    type Output = Self;
    
    fn rem(self, other: Self) -> Self {
        
        if self == Int::zero() {
            Int::zero()
        }
        
        else if other == Int::zero() {
            panic!("a/0 is undefined!")
        }
        
        else {
    
            let (_, r) = divisor(self.magnitude, other.magnitude);
    
            if self.sign && r != vec![Bit::Zero] {
                Int { magnitude: r, sign: true }
            } else {
                Int { magnitude: r, sign: false }
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