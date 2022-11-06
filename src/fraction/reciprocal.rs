use crate::Fraction;

impl Fraction {
    pub fn reciprocal(&self) -> Fraction {
        Fraction(self.1.clone(), self.0.clone())
    }
}
