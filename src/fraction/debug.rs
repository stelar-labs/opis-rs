use crate::Fraction;
use std::fmt;

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0.to_dec(), self.1.to_dec())
    }
}
