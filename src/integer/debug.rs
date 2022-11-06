use crate::{Bit, Integer};
use std::fmt;

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0
            .iter()
            .fold(String::new(),|mut acc, x| {
                match x {
                    Bit::One => acc.push('1'),
                    Bit::Zero => acc.push('0')
                }
                acc
            })
        )
    }
}
