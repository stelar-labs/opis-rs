use crate::Int;
use std::ops::Not;

impl Not for Int {

    type Output = Self;
    
    fn not(mut self) -> Self::Output {

        self.sign = !self.sign;
        
        self.magnitude = self.magnitude
            .iter()
            .map(|x| !x)
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