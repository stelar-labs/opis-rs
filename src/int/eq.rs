use crate::Int;

impl PartialEq for Int {
    fn eq(&self, other: &Self) -> bool {
        self.magnitude == other.magnitude && self.sign == other.sign
    }
}

impl Eq for Int {}