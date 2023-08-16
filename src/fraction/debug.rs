use crate::Fraction;
use std::fmt;

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fraction")
            .field("value", &format!("{}/{}", self.0.to_dec(), self.1.to_dec()))
            .field("hex", &format!("{}/{}", self.0.to_hex(), self.1.to_hex()))
            .field("bin", &format!("{}/{}", self.0.to_bin(), self.1.to_bin()))
            .finish()
    }
}
