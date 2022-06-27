use crate::{Bit, Int};
use std::error::Error;

impl Int {

    pub fn from_bin(arg: &str) -> Result<Self, Box<dyn Error>> {

        if arg.len() > 3 {

            let (_, last) = arg.split_at(2);

            println!(" * last: {}", last);
            
            let mut bin_str: Vec<_> = last.split("").collect();
            
            bin_str.retain(|&x| x != "");

            let mut bits = Vec::new();

            for bit in bin_str {
                
                match bit {

                    "0" => bits.push(Bit::Zero),

                    "1" => bits.push(Bit::One),

                    _ => Err("Internal error!")?

                }

            }

            while bits.len() > 2 && bits[0] == bits[1] {
                bits.remove(0);
            }

            Ok(Int { bits })

        } else {

            Err("Internal error!")?
            
        }

    }

    pub fn to_bin(&self) -> String {

        let mut result: String = String::from("b'");

        for bit in &self.bits {

            match bit {

                Bit::Zero => result.push('0'),

                Bit::One => result.push('1')

            }
        };

        result

    }

}
