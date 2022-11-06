use crate::Bit;
use std::fmt;

impl fmt::Debug for Bit {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}",  match self { Bit::One => '1', Bit::Zero => '0' })

    }
    
}
