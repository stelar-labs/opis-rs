use crate::{Bit, Int};
use std::error::Error;

impl Int {

    pub fn from_bytes(arg: &[u8]) -> Result<Self, Box<dyn Error>> {

        let binary_string =

            arg
            .iter()
            .fold(
                String::from("b'"),
                |acc, x|
                format!("{}{:08b}", acc, x)
            );

        Int::from_bin(&binary_string)

    }

    pub fn to_bytes(&self) -> Vec<u8> {

        let mut result = Vec::new();

        let rem = self.bits.len() % 8;

        let fill = 8 - rem;

        let byte_count = 
        
            if rem == 0 {

                self.bits.len() / 8

            } else {
                
                (self.bits.len() / 8) + 1
            
            };

        

        for byte_index in 0..byte_count {

            let mut byte_bits = Vec::new();

            let mut i =
            
                if byte_index == 0 {

                    if rem != 0 {

                        byte_bits = vec![self.bits[0]; fill]

                    };

                    0

                } else {

                    (byte_index * 8) - fill

                };

            while byte_bits.len() < 8 {

                byte_bits.push(self.bits[i]);

                i += 1;

            }

            let byte_int = Int { bits: byte_bits };

            let byte = u8::from_str_radix(&byte_int.to_bin(), 2).unwrap();

            result.push(byte)

        }
        
        result

    }

    pub fn to_ext_bits(&self, length: usize) -> Vec<Bit> {

        let ext_length =

            if self.bits.len() < length {
                length - self.bits.len()
            } else {
                0
            };

        let ext_bits = vec![self.bits[0]; ext_length];

        [ext_bits, self.bits.clone()].concat()

    }

    pub fn to_ext_bytes(&self, length: usize) -> Vec<u8> {

        let bits = self.to_ext_bits(length * 8);

        Int { bits }.to_bytes()

    }

}
