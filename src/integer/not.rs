use crate::Integer;
use std::ops::Not;

use super::Digit;

impl Not for Integer {
    type Output = Self;
    
    fn not(self) -> Self::Output {
        !&self
    }
}

impl Not for &Integer {
    type Output = Integer;

    fn not(self) -> Integer {
        let inverted_digits = self.digits.iter()
            .map(|&digit| !digit)
            .collect::<Vec<Digit>>();

        Integer { digits: inverted_digits }
    }
}
