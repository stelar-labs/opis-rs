mod addition;
mod comparison;
mod debug;
mod display;
mod division;
mod equality;
mod inversion;
mod multiplication;
mod reciprocal;
mod reduction;
mod string_conversion;
mod subtraction;
mod type_conversion;
use crate::{Fraction, Bit, Integer};

impl Default for Fraction {
    fn default() -> Self {
        Fraction::zero()
    }
}

impl Fraction {

    pub fn sign(&self) -> bool {
        match (self.0.0[0], self.1.0[0]) {
            (Bit::One, Bit::Zero) => true,
            (Bit::Zero, Bit::One) => true,
            _ => false
        }
    }

    pub fn zero() -> Fraction { Fraction(Integer::zero(), Integer::one()) }

    pub fn one() -> Fraction { Fraction(Integer::one(), Integer::one()) }

    pub fn neg_one() -> Fraction { Fraction(Integer::neg_one(), Integer::one()) }
    
}