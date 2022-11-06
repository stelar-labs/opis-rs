use crate::Fraction;

impl PartialEq for Fraction {
    fn eq(&self, b: &Self) -> bool {
        self.0 == b.0 && self.1 == b.1
    }
}

impl Eq for Fraction {}
