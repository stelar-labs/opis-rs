use crate::Bit;
use std::ops::Not;

impl Not for Bit {

    type Output = Self;
    
    fn not(self) -> Self::Output {
        not(&self)
    }

}

impl Not for &Bit {
    
    type Output = Bit;
    
    fn not(self) -> Bit {
        not(self)
    }
    
}

fn not(a: &Bit) -> Bit {
    
    match a {
            
        Bit::One => Bit::Zero,
        
        Bit::Zero => Bit::One
    
    }

}
