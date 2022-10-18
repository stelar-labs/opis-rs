use crate::Bit;
// EQUALITY
impl PartialEq for Bit {
    fn eq(&self, b: &Self) -> bool {
        match (self, b) {
            (Bit::Zero, Bit::Zero) => true,
            (Bit::One, Bit::One) => true,
            _ => false
        }
    }
}
impl Eq for Bit {}
// AND
use std::ops::BitAnd;

impl BitAnd for Bit {
    type Output = Self;
    fn bitand(self, b: Self) -> Self::Output {
        and(&self, &b)
    }
}

impl BitAnd for &Bit {
    type Output = Bit;
    fn bitand(self, b: Self) -> Bit {
        and(self, b)
    }
}

fn and(a: &Bit, b: &Bit) -> Bit {
    if a == &Bit::One && b == &Bit::One {
        Bit::One
    } else {
        Bit::Zero
    }
}

// NOT
use std::ops::Not;

impl Not for Bit {
    type Output = Self;
    fn not(self) -> Self::Output {
        not(&self)
    }
}

impl Not for &Bit {
    type Output = Bit;
    fn not(self) -> Bit {
        not(self)
    }
}

fn not(a: &Bit) -> Bit {
    match a {
        Bit::One => Bit::Zero,
        Bit::Zero => Bit::One
    }
}

// OR
use std::ops::BitOr;

impl BitOr for Bit {
    type Output = Self;
    fn bitor(self, b: Self) -> Self::Output {
        or(&self, &b)
    }
}

impl BitOr for &Bit {
    type Output = Bit;
    fn bitor(self, b: Self) -> Bit {
        or(self, b)
    }
}

fn or(a: &Bit, b: &Bit) -> Bit {
    if a == &Bit::Zero &&  b == &Bit::Zero {
        Bit::Zero
    } else {
        Bit::One
    }
}

// XOR
use std::ops::BitXor;

impl BitXor for Bit {
    type Output = Self;
    fn bitxor(self, b: Self) -> Self::Output {
        xor(&self, &b)
    }
}

impl BitXor for &Bit {
    type Output = Bit;
    fn bitxor(self, b: Self) -> Bit {
        xor(self, b)
    }
}

fn xor(a: &Bit, b: &Bit) -> Bit {
    match (a, b) {      
        (Bit::Zero, Bit::One) => Bit::One,
        (Bit::One, Bit::Zero) => Bit::One,
        _ => Bit::Zero
    }
}

// ADDITION
use std::ops::Add;

impl Add for Bit {
    type Output = (Self, Self);
    fn add(self, b: Self) -> (Self, Self) {
        add(&self, &b)
    }
}

impl Add for &Bit {
    type Output = (Bit, Bit);
    fn add(self, b: Self) -> (Bit, Bit) {
        add(self, b)
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