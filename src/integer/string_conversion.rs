use std::error::Error;

use crate::{Bit, Integer};


impl Integer {

    pub fn from_bin(str: &str) -> Result<Self, Box<dyn Error>> {

        if str.len() > 0 {
            
            let bit_chars = str.chars().collect::<Vec<_>>();

            let mut bits = Vec::new();

            for bit in bit_chars {

                match bit {
                    '0' => bits.push(Bit::Zero),
                    '1' => bits.push(Bit::One),
                    _ => Err("Internal error!")?
                }

            }

            let mut result = Integer(bits);

            result.clean();

            Ok(result)

        } else {

            Ok(Integer::zero())

        }

    }

    pub fn to_bin(&self) -> String {

        self.0
            .iter()
            .fold(
                String::new(), |mut acc, x| {
                    match x {
                        Bit::Zero => acc.push('0'),
                        Bit::One => acc.push('1')
                    }
                    acc
                }
            )
    }
    
    pub fn from_dec(str: &str) -> Result<Self, Box<dyn Error>> {

        if str.len() > 0 {
            
            let (first, last) = str.split_at(1);
            
            let (dec_str, sign) =
                match first {
                    "-" => (last, true),
                    _ => (str, false)
                };

            let mut bits: Vec<Bit> = Vec::new();

            let mut i_str = dec_str.to_string();

            while i_str != "0".to_string() {

                let (s_half, rem) = half(&i_str);

                i_str = s_half;

                match rem {
                    0 => bits.push(Bit::Zero),
                    1 => bits.push(Bit::One),
                    _ => Err("Internal error!")?
                }

            }

            bits.push(Bit::Zero);

            bits.reverse();
            
            if bits.is_empty() {
                    
                Ok(Integer::zero())

            } else {
                
                if sign {
                    
                    Ok(Integer(bits).negate())

                } else {

                    Ok(Integer(bits))

                }

            }
            
        } else {

            Err("Internal error!")?
        
        }

    }

    pub fn to_dec(&self) -> String {
        
        if self.0[0] == Bit::One {

            format!("-{}", bits_to_dec_string(&self.negate().0[1..]))

        } else {

            bits_to_dec_string(&self.0[1..])

        }

    }

    pub fn to_hex(&self) -> String {

        let int_bytes: Vec<u8> = self.into();
        
        int_bytes
            .iter()
            .fold(
                String::new(), |mut acc, x| {
                    acc.push_str(&format!("{:02x}", x));
                    acc
                }
            )

    }

    pub fn from_hex(str: &str) -> Result<Self, Box<dyn Error>> {

        if str.len() > 0 {

            let mut bin_str = String::new();
            
            let hex_chars = str.chars().collect::<Vec<_>>();

            for hex_char in hex_chars {

                match hex_char {
                    '0' => bin_str.push_str("0000"),
                    '1' => bin_str.push_str("0001"),
                    '2' => bin_str.push_str("0010"),
                    '3' => bin_str.push_str("0011"),
                    '4' => bin_str.push_str("0100"),
                    '5' => bin_str.push_str("0101"),
                    '6' => bin_str.push_str("0110"),
                    '7' => bin_str.push_str("0111"),
                    '8' => bin_str.push_str("1000"),
                    '9' => bin_str.push_str("1001"),
                    'A' => bin_str.push_str("1010"),
                    'B' => bin_str.push_str("1011"),
                    'C' => bin_str.push_str("1100"),
                    'D' => bin_str.push_str("1101"),
                    'E' => bin_str.push_str("1110"),
                    'F' => bin_str.push_str("1111"),
                    _ => Err("Internal error!")?             
                }

            }

            Integer::from_bin(&bin_str)

        } else {
            
            Ok(Integer::zero())
        
        }

    }

}

fn bits_to_dec_string(bits: &[Bit]) -> String {

    bits.iter().fold("0".to_string(), |mut acc, x| { 
        acc = double(&acc);
        if x == &Bit::One {
            acc = add_one(&acc);
        }
        acc
    })

}

fn double(s: &str) -> String {

    let mut split: Vec<_> = s.split("").collect();
    
    split.retain(|&x| x != "");

    split.reverse();

    let mut carry: u8 = 0;

    let mut current_number: Vec<u8> = Vec::new();

    for i in split {

        let mut res: u8 = carry;

        let n: u8 = u8::from_str_radix(&i, 10).unwrap();

        let d: u8 = n * 2;

        if d > 9 {

            res += d - 10;

            carry = 1;

        } else {

            res += d;

            carry = 0;

        }

        current_number.push(res)

    }

    if carry == 1 {

        current_number.push(1)
    
    }

    current_number.reverse();

    let num_str: String = current_number
        .iter()
        .fold(
            String::new(),
            |acc, x|
            format!("{}{}", acc, x)
        );

    num_str

}

fn add_one(s: &str) -> String {

    let mut split: Vec<_> = s.split("").collect();
    
    split.retain(|&x| x != "");

    split.reverse();

    let mut carry: u8 = 1;

    let mut current_number: Vec<u8> = Vec::new();

    for i in split {

        let n: u8 = u8::from_str_radix(&i, 10).unwrap();

        let mut sum: u8 = carry + n;

        if sum == 10 {

            sum = 0;

            carry = 1;

        } else {

            carry = 0;

        }

        current_number.push(sum)

    }

    if carry == 1 {

        current_number.push(1)
    
    }

    current_number.reverse();

    let num_str: String = current_number
    .iter()
    .fold(
        String::new(),
        |acc, x|
        format!("{}{}", acc, x)
    );

    num_str

}

fn half(s: &str) -> (String, u8) {

    let mut split: Vec<_> = s.split("").collect();
    
    split.retain(|&x| x != "");

    let split_size = split.len();

    let mut res: String = String::new();

    let mut rem: u8 = 0;

    for i in split {

        let n_str = format!("{}{}", rem, i);

        let n = u8::from_str_radix(&n_str, 10).unwrap();

        let d = n/2;

        if res == String::new() {

            if split_size == 1 {

                res.push_str(&d.to_string())

            } else {
            
                if d != 0 {
                    
                    res.push_str(&d.to_string())
                
                }

            }

        } else {

            res.push_str(&d.to_string())

        }
        
        rem = n%2;

    }

    (res, rem)

}
