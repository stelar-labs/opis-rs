use std::ops::Not;

use crate::Bit;
use crate::Int;

impl Not for Int {

    type Output = Self;
    
    fn not(mut self) -> Self::Output {
        
        self.bits = self.bits
            .iter()
            .map(|x| {
                match x {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::One
                }
            })
            .collect();

        self
    }

}

impl Not for &Int {

    type Output = Int;
    
    fn not(self) -> Int {
        !self.clone()
    }
    
}