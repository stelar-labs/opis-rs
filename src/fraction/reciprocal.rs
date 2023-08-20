use std::error::Error;

use crate::Fraction;

impl Fraction {
    pub fn reciprocal(&self) -> Result<Fraction, Box<dyn Error>> {

        if self == &Fraction::zero() {
            Err("Only for Non Zero!")?
        } else {
            Ok(Fraction(self.1.clone(), self.0.clone()))
        }
    }
}
