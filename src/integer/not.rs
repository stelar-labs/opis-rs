use crate::Integer;
use std::ops::Not;

impl Not for Integer {
    type Output = Self;
    fn not(self) -> Self::Output {
        !&self
    }
}

impl Not for &Integer {
    type Output = Integer;
    fn not(self) -> Integer {
        let not_bits = self.0.iter().map(|x| !x).collect();
        Integer(not_bits)
    }
}
