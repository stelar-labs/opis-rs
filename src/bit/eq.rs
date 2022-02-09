use crate::Bit;

impl PartialEq for Bit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => true,
            (Bit::One, Bit::One) => true,
            _ => false
        }
    }
}

impl Eq for Bit {}