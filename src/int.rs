use crate::Bit;
use crate::Int;

pub mod cmp;
pub mod eq;

pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod rem;

pub mod not;
pub mod and;
pub mod or;
pub mod xor;

pub mod pow;

mod base2;
mod base10;
mod base16;
mod bytes;

impl Int {

    pub fn zero() -> Self {
        Int { magnitude: vec![Bit::Zero], sign: false }
    }

    pub fn one() -> Self {
        Int { magnitude: vec![Bit::One], sign: false }    
    }

    pub fn from_binary(input: &str) -> Self {

        if input.len() > 3 {

            let (first, last) = input.split_at(2);

            match first {
                "b'" => Int { magnitude: base2::from(last), sign: false },
                _ => panic!("Binary string must start with b'!")
            }

        } else {
            panic!("String is too short!")
        }

    }

    pub fn from_bytes(input: &Vec<u8>) -> Self {
        Int { magnitude: bytes::from(input), sign: false }
    }

    pub fn from_decimal(input: &str) -> Self {

        if input.len() > 1 {

            let (first, last) = input.split_at(1);

            match first {
                "-" => Int { magnitude: base10::from(last), sign: true },
                _ => Int { magnitude: base10::from(input), sign: false }
            }

        } else {
            panic!("String is too short!")
        }
    }

    pub fn from_hex(input: &str) -> Self {

        if input.len() > 3 {

            let (first, last) = input.split_at(2);

            match first {
                "0x" => Int { magnitude: base16::from(last), sign: false },
                _ => panic!("Hex string must start with 0x!")
            }

        } else {
            panic!("String is too short!")
        }

    }

    pub fn to_binary(self) -> String {
        base2::to(self.magnitude)
    }

    pub fn to_decimal(self) -> String {
        base10::to(self.magnitude)
    }

    pub fn to_hex(self) -> String {
        base16::to(self.to_bytes())
    }

    pub fn to_bytes(self) -> Vec<u8> {
        bytes::to(self.magnitude)
    }

    pub fn to_ext_bytes(self, length: usize) -> Vec<u8> {
        bytes::to_ext(self.magnitude, length)
    }

    pub fn negative(&mut self) {
        if self.sign == false {
            self.sign = true
        }
    }

    pub fn positive(&mut self) {
        if self.sign == true {
            self.sign = false
        }
    }

}