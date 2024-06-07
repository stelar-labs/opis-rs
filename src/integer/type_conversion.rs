// use crate::{Integer, Bit};

// impl From<&[Bit]> for Integer{

//     fn from(bits: &[Bit]) -> Self {

//         if bits.is_empty() {

//             Integer::zero()

//         } else if bits.len() == 1 {

//             Integer(vec![Bit::Zero, bits[0]])

//         } else {

//             Integer(bits.to_vec())

//         }
//     }
// }



use crate::Integer;

use super::Digit;

impl From<Vec<u8>> for Integer{
    fn from(value: Vec<u8>) -> Self {
        Integer::from(&value[..])
    }
}

impl From<&[u8]> for Integer{

    fn from(bytes: &[u8]) -> Self {
        let most_significant_bit = bytes.last().map_or(0, |&b| b) & 0x80 != 0;
        let most_significant_digit = if most_significant_bit { u8::MAX } else { u8::MIN };
        let mut temp_array = [most_significant_digit; std::mem::size_of::<Digit>()];
        let mut digits = Vec::new();
        let mut index = 0;

        for &byte in bytes {
            temp_array[index] = byte;
            index += 1;

            if index == std::mem::size_of::<Digit>() {
                digits.push(Digit::from_le_bytes(temp_array));
                index = 0;
                temp_array = [most_significant_digit; std::mem::size_of::<Digit>()];
            }
        }

        if index != 0 {
            digits.push(Digit::from_le_bytes(temp_array));
        }

        Integer { digits }
        
    }

}

impl Into<Vec<u8>> for Integer {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}

impl Into<Vec<u8>> for &Integer{
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        for digit in &self.digits {
            let bytes = digit.to_le_bytes();
            result.extend_from_slice(&bytes);
        }
        result
    }
}

// pub fn bits_to_bytes(bits: &[Bit]) -> Vec<u8> {

//     let mut bytes = vec![];

//     let r = bits.len() % 8;

//     let mut byte = if r != 0 {
        
//         vec![bits[0]; 8 - r]

//     } else {

//         vec![]

//     };

//     bits
//     .iter()
//     .for_each(|&i| {

//         byte.push(i);

//         if byte.len() == 8 {

//             let p: Vec<String> = byte.iter().map(|x| format!("{:?}",x)).collect();
        
//             let q = p.concat();

//             bytes.push(u8::from_str_radix(&q, 2).unwrap());

//             byte.clear();

//         }

//     });

//     bytes

// }

impl From<&u8> for Integer {
    fn from(value: &u8) -> Self {
        let digit = *value as Digit;
        Integer {
            digits: vec![digit],
        }
    }
}

impl From<&u16> for Integer {
    fn from(value: &u16) -> Self {
        let digit = *value as Digit;
        Integer {
            digits: vec![digit],
        }
    }
}

impl From<&u32> for Integer {
    fn from(value: &u32) -> Self {
        let digit = *value as Digit;
        Integer {
            digits: vec![digit],
        }
    }
}

impl From<&u64> for Integer {
    fn from(value: &u64) -> Self {
        #[cfg(target_pointer_width = "64")]
        {
            let digit = *value as Digit;
            Integer {
                digits: vec![digit],
            }
        }

        #[cfg(target_pointer_width = "32")]
        {
            let lower = (*value & 0xFFFF_FFFF) as u32;
            let upper = (*value >> 32) as u32;
            let digits = if upper != 0 { vec![lower, upper] } else { vec![lower] };
            Integer {
                digits,
            }
        }
    }
}

impl TryInto<u8> for &Integer {
    type Error = &'static str;

    fn try_into(self) -> Result<u8, Self::Error> {
        if self.digits.len() > 1 || (self.digits.len() == 1 && self.digits[0] > u8::MAX as Digit) {
            Err("Integer value too large to fit into a u8")
        } else if self.digits.is_empty() {
            Err("Integer has no digits")
        } else {
            Ok(self.digits[0] as u8)
        }
    }
}

impl TryInto<u16> for &Integer {
    type Error = &'static str;

    fn try_into(self) -> Result<u16, Self::Error> {
        if self.digits.len() > 1 || (self.digits.len() == 1 && self.digits[0] > u16::MAX as Digit) {
            Err("Integer value too large to fit into a u16")
        } else if self.digits.is_empty() {
            Err("Integer has no digits")
        } else {
            Ok(self.digits[0] as u16)
        }
    }
}

impl TryInto<u32> for &Integer {
    type Error = &'static str;

    fn try_into(self) -> Result<u32, Self::Error> {
        if self.digits.len() > 1 || (self.digits.len() == 1 && self.digits[0] > u32::MAX as Digit) {
            Err("Integer value too large to fit into a u32")
        } else if self.digits.is_empty() {
            Err("Integer has no digits")
        } else {
            Ok(self.digits[0] as u32)
        }
    }
}

impl TryInto<u64> for &Integer {
    type Error = &'static str;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self.digits.len() {
            0 => Err("Integer has no digits"),
            1 => Ok(self.digits[0] as u64),
            2 => {
                #[cfg(target_pointer_width = "32")]
                {
                    // Combine two u32 digits into one u64 value on 32-bit systems
                    let combined = ((self.digits[1] as u64) << 32) | (self.digits[0] as u64);
                    Ok(combined)
                }
                #[cfg(not(target_pointer_width = "32"))]
                {
                    Err("Integer value too large to fit into a u64")
                }
            }
            _ => Err("Integer value too large to fit into a u64")
        }
    }
}

// #[cfg(test)]
// mod tests {
    
//     use super::*;

//     #[test]
//     fn test_int_into_bytes() {

//         let int_bytes: Vec<u8> = Integer::three().into();

//         assert_eq!(int_bytes, vec![3]);

//     }

//     #[test]
//     fn test_large_int_into_bytes() {

//         let i = Integer::from_dec("1000").unwrap();

//         let i_bytes: Vec<u8> = i.into();

//         assert_eq!(i_bytes, vec![3, 232]);

//     }

// }
