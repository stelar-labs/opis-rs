use crate::Int;
use std::error::Error;

impl Int {

    pub fn from_hex(hex_str: &str) -> Result<Self, Box<dyn Error>> {

        if hex_str.len() > 3 {
            
            let (first, last) = hex_str.split_at(2);
        
            match first {

                "0x" => {
                    
                    let mut hex_str_vec: Vec<_> = last.split("").collect();
                    
                    hex_str_vec.retain(|&x| x != "");

                    let binary_string =

                        hex_str_vec
                            .iter()
                            .fold(
                                String::from("b'"),
                                |acc, x|
                                {
                                    
                                    let b = u8::from_str_radix(x, 16).unwrap();

                                    format!("{}{:04b}", acc, b)
                                
                                }
                            );

                    Int::from_bin(&binary_string)

                },

                _ => Err("Internal error!")?
            
            }

        } else {
            
            Err("Internal error!")?
        
        }

    }

    pub fn to_hex(&self) -> String {

        let bytes = self.to_bytes();
        
        let mut result = String::from("0x");

        bytes
            .iter()
            .for_each(|x| {

                let hex_str = format!("{:02X}", x);

                result.push_str(&hex_str)

            });
            
        result

    }

}
