use crate::{Integer, Bit};


impl Integer {
    pub fn posate(&mut self) {
        if self.0[0] == Bit::One {
            *self = self.inversion()
        }
    }
    pub fn negate(&mut self) {
        if self.0[0] == Bit::Zero {
            *self = self.inversion()
        }
    }
}