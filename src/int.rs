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
pub mod modulo;
pub mod mod_inv;

mod base2;
mod base10;
mod base16;
mod bytes;

impl Int {

    pub fn zero() -> Self {
        Int {bits: vec![Bit::Zero, Bit::Zero]}
    }

    pub fn one() -> Self {
        Int {bits: vec![Bit::Zero, Bit::One]}
    }

    pub fn from(input: &str) -> Self {

        if input.len() > 2 {

            let (first, last) = input.split_at(2);

            match first {
                
                "b'" => Int {bits: base2::from(last)},
                
                "0x" => {
                
                    if last.len() > 2 {

                        let (sign, mag) = last.split_at(2);
                
                        match sign {
                            "00" => Int {bits: [vec![Bit::Zero], base16::from(mag)].concat()},
                            "01" => Int {bits: [vec![Bit::One], base16::from(mag)].concat()},
                            _ => panic!("Unsupported base16 format!")
                        }

                    }

                    else {
                        panic!("Unsupported base16 format!")
                    }
                    
                },
                _ => Int {bits: base10::from(input)}
            }


        } else {
            Int {bits: base10::from(input)}
        }

    }

    pub fn to(self, radix: u8) -> String {
        match radix {
            2 => base2::to(self.bits),
            10 => base10::to(self.bits),
            16 => base16::to(self.to_bytes()),
            _ => panic!("Unsupported radix!")
        }
    }

    pub fn from_bytes(b: &Vec<u8>) -> Self {

        let bits = bytes::from(&b[1..].to_vec());

        match b[0] {
            0 => Int {bits: [vec![Bit::Zero], bits].concat()},
            1 => Int {bits: [vec![Bit::One], bits].concat()},
            _ => panic!("Unsupported sign!")
        }
        
    }

    pub fn to_bytes(self) -> Vec<u8> {
        
        let b = bytes::to(self.bits[1..].to_vec());

        match self.bits[0] {
            Bit::One => [vec![1], b].concat(),
            Bit::Zero => [vec![0], b].concat()
        }

    }

}