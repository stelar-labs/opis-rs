use crate::Bit;
use std::ops::Add;

impl Add for Bit {
    type Output = (Self, Self);
    fn add(self, b: Self) -> (Self, Self) {
        add(&self, &b)
    }
}

impl Add<&Bit> for Bit {
    type Output = (Self, Self);
    fn add(self, b: &Bit) -> (Self, Self) {
        add(&self, b)
    }
}

impl Add for &Bit {
    type Output = (Bit, Bit);
    fn add(self, b: Self) -> (Bit, Bit) {
        add(self, b)
    }
}

impl Add<Bit> for &Bit {
    type Output = (Bit, Bit);
    fn add(self, b: Bit) -> (Bit, Bit) {
        add(self, &b)
    }
}

fn add(a: &Bit, b: &Bit) -> (Bit, Bit) {
    match (a, b) {
        (Bit::Zero, Bit::Zero) => (Bit::Zero, Bit::Zero),
        (Bit::Zero, Bit::One) => (Bit::Zero, Bit::One),
        (Bit::One, Bit::Zero) => (Bit::Zero, Bit::One),
        (Bit::One, Bit::One) => (Bit::One, Bit::Zero)
    }
}
