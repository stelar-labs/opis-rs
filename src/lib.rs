
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, Rem, Not, BitAnd, BitOr, BitXor};

mod arithmetic;
mod conversion;
mod implements;
mod operators;

#[derive(Clone, Debug)]
pub enum Bit { Zero, One }

impl PartialEq for Bit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => true,
            (Bit::One, Bit::One) => true,
            _ => false
        }
    }
}

impl Eq for Bit {}

#[derive(Clone, Debug)]
pub struct Int { pub bits: Vec<Bit>, pub sign: bool }

impl Int {

    pub fn zero() -> Self {
        Int { bits: vec![Bit::Zero], sign: false }
    }

    pub fn one() -> Self {
        Int { bits: vec![Bit::One], sign: false }
    }

    pub fn negate(mut self) -> Self {
        self.sign = true; self
    }

    pub fn from_str(s: &str, r: u8) -> Self {
        match r {
            2 => Int { bits: conversion::base2::from(s), sign: false },
            10 => Int { bits: conversion::base10::from(s), sign: false },
            16 => Int { bits: conversion::base16::from(s), sign: false },
            _ => panic!("Unsupported radix!")
        }
    }

    pub fn to_str(self, r: u8) -> String {
        match r {
            2 => conversion::base2::to(self.bits),
            10 => conversion::base10::to(self.bits),
            16 => conversion::base16::to(self.to_bytes()),
            _ => panic!("Unsupported radix!")
        }
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Int {
        Int { bits: conversion::bytes::from(bytes), sign: false }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        conversion::bytes::to(self.bits)
    }

}

impl Add for Int {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        implements::add::main(self, b)
    }
}

impl Add for &Int {
    type Output = Int;
    fn add(self, b: Self) -> Int {
        implements::add::main(self.clone(), b.clone())
    }
}

impl AddAssign for Int {
    fn add_assign(&mut self, b: Self) {
        *self = self.clone() + b
    }
}

impl Sub for Int {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        implements::sub::main(self.clone(), b.clone())
    }
}

impl Sub for &Int {
    type Output = Int;
    fn sub(self, b: Self) -> Int {
        implements::sub::main(self.clone(), b.clone())
    }
}

impl SubAssign for Int {
    fn sub_assign(&mut self, b: Self) {
        *self = self.clone() - b
    }
}

impl Mul for Int {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        implements::mul::main(self, b)
    }
}

impl Mul for &Int {
    type Output = Int;
    fn mul(self, b: Self) -> Int {
        implements::mul::main(self.clone(), b.clone())
    }
}

impl Div for Int {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        implements::div::main(self, b)
    }
}

impl Div for &Int {
    type Output = Int;
    fn div(self, b: Self) -> Int {
        implements::div::main(self.clone(), b.clone())
    }
}

impl Rem for Int {
    type Output = Self;
    fn rem(self, b: Self) -> Self {
        implements::rem::main(self, b)
    }
}

impl Rem for &Int {
    type Output = Int;
    fn rem(self, b: Self) -> Int {
        implements::rem::main(self.clone(), b.clone())
    }
}

impl Not for Int {
    type Output = Self;
    fn not(self) -> Self::Output {
        implements::not::main(self)
    }
}

impl Not for &Int {
    type Output = Int;
    fn not(self) -> Int {
        implements::not::main(self.clone())
    }
}

impl BitAnd for Int {
    type Output = Self;
    fn bitand(self, b: Self) -> Self::Output {
        implements::and::main(self, b)
    }
}

impl BitAnd for &Int {
    type Output = Int;
    fn bitand(self, b: Self) -> Int {
        implements::and::main(self.clone(), b.clone())
    }
}

impl BitOr for Int {
    type Output = Self;
    fn bitor(self, b: Self) -> Self::Output {
        implements::or::main(self, b)
    }
}

impl BitOr for &Int {
    type Output = Int;
    fn bitor(self, b: Self) -> Int {
        implements::or::main(self.clone(), b.clone())
    }
}

impl BitXor for Int {
    type Output = Self;
    fn bitxor(self, b: Self) -> Self::Output {
        implements::xor::main(self, b)
    }
}

impl BitXor for &Int {
    type Output = Int;
    fn bitxor(self, b: Self) -> Int {
        implements::xor::main(self.clone(), b.clone())
    }
}

impl Ord for Int {
    fn cmp(&self, b: &Self) -> Ordering {
        implements::cmp::main(self, b)
    }
}

impl PartialOrd for Int {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}

impl PartialEq for Int {
    fn eq(&self, b: &Self) -> bool {
        self.bits == b.bits && self.sign == b.sign
    }
}

impl Eq for Int {}

pub fn mod_inv(a: &Int, m: &Int) -> Int {
    arithmetic::mod_inv::main(a, m)
}

pub fn modulo(a: &Int, b: &Int) -> Int {
    arithmetic::modulo::main(a, b)
}

pub fn pow(a: &Int, e:&Int) -> Int {
    arithmetic::pow::main(a, e)
}
