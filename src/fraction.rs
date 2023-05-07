mod addition;
mod comparison;
mod debug;
mod division;
mod equality;
mod inversion;
mod multiplication;
mod reciprocal;
mod reduction;
mod string_conversion;
mod subtraction;
mod type_conversion;
use crate::{Fraction, Bit};

impl Fraction {

    pub fn sign(&self) -> bool {

        match (self.0.0[0], self.1.0[0]) {

            (Bit::One, Bit::Zero) => false,

            (Bit::Zero, Bit::One) => false,

            _ => true

        }

    }
    
}