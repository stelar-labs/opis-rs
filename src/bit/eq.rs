use crate::Bit;

impl PartialEq for Bit {
    fn eq(&self, b: &Self) -> bool {
        match (self, b) {
            (Bit::Zero, Bit::Zero) => true,
            (Bit::One, Bit::One) => true,
            _ => false
        }
    }
}

impl Eq for Bit {}
