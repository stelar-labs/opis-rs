use std::fmt;

use crate::Fraction;

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.0.to_dec(), self.1.to_dec())
    }
}