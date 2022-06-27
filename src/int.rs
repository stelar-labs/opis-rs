mod basic;
mod bits;
mod convert;
mod modular;
mod lfsr;
mod neg;
mod pos;
mod pow;
use crate::{Bit, Int};

impl Int {

    pub fn zero() -> Self {
        Int { bits: vec![Bit::Zero, Bit::Zero] }
    }

    pub fn one() -> Self {
        Int { bits: vec![Bit::Zero, Bit::One] }
    }

    pub fn neg_one() -> Self {
        Int { bits: vec![Bit::One, Bit::One] }
    }

    pub fn two() -> Self {
        Int { bits: vec![Bit::Zero, Bit::One, Bit::Zero] }
    }

    pub fn three() -> Self {
        Int { bits: vec![Bit::Zero, Bit::One, Bit::One] }
    }

}
