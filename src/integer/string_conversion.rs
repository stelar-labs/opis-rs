use std::error::Error;

use crate::{Bit, Integer};

impl TryFrom<&str> for Integer {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {

        if value.len() < 3 {

            Integer::from_dec(value)

        } else {

            let (first, last) = value.split_at(2);

            match &first.to_lowercase()[..] {

                "b'" => Integer::from_bin(last),

                "0x" => Integer::from_hex(last),

                _ => Integer::from_dec(value)

            }

        }

    }
    
}

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

            result.truncate();

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

            if dec_str == "0" {
                return Ok(Integer::zero());
            } else {

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
                
                if sign {
                    Ok(Integer(bits).inversion())
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

            format!("-{}", bits_to_dec_string(&self.inversion().0[1..]))

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

    pub fn from_hex(arg: &str) -> Result<Self, Box<dyn Error>> {

        if arg.len() > 0 {

            let upper_str = arg.to_uppercase();
            
            let hex_chars = upper_str.chars().collect::<Vec<_>>();

            let bin_ops: Vec<Option<String>> = hex_chars
                .iter()
                .map(|x| {

                    match x {
                        '0' => Some("0000".to_string()),
                        '1' => Some("0001".to_string()),
                        '2' => Some("0010".to_string()),
                        '3' => Some("0011".to_string()),
                        '4' => Some("0100".to_string()),
                        '5' => Some("0101".to_string()),
                        '6' => Some("0110".to_string()),
                        '7' => Some("0111".to_string()),
                        '8' => Some("1000".to_string()),
                        '9' => Some("1001".to_string()),
                        'A' => Some("1010".to_string()),
                        'B' => Some("1011".to_string()),
                        'C' => Some("1100".to_string()),
                        'D' => Some("1101".to_string()),
                        'E' => Some("1110".to_string()),
                        'F' => Some("1111".to_string()),
                        _ => None            
                    }

                })
                .collect();

            let err_check = bin_ops
                .iter()
                .any(|x| {
                    match x {
                        Some(_) => false,
                        None => true
                    }
                });

            if err_check {

                Err("Internal error!")?

            } else {

                let bin_strs: Vec<String> = bin_ops.iter().map(|x| x.clone().unwrap()).collect();

                let bin_str = bin_strs.concat();

                Integer::from_bin(&bin_str)

            }

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

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_int_try_from_str_0() {
        assert_eq!(
            Integer::try_from("3").unwrap(),
            Integer::three()
        )
    }

    #[test]
    fn test_int_try_from_str_1() {
        assert_eq!(
            Integer::try_from("b'011").unwrap(),
            Integer::three()
        )
    }


    #[test]
    fn test_int_try_from_str_2() {
        assert_eq!(
            Integer::try_from("0x03").unwrap(),
            Integer::three()
        )
    }


}