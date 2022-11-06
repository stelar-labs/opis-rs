mod addition;
mod comparison;
mod debug;
mod division;
mod equality;
mod multiplication;
mod reciprocal;
mod reduce;
mod string_conversion;
mod subtraction;
mod type_conversion;
use crate::{Fraction, Bit};

impl Fraction {

    pub fn is_negative(&self) -> bool {
        match (self.0.0[0], self.1.0[0]) {
            (Bit::One, Bit::Zero) => true,
            (Bit::Zero, Bit::One) => true,
            _ => false
        }
    }
}