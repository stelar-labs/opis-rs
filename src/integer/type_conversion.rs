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

// impl From<&u8> for Integer{
//     fn from(value: &u8) -> Self {
//         Integer::from(&value.to_le_bytes()[..])
//     }
// }

// impl Into<u8> for &Integer{
//     fn into(self) -> u8 {
//         u8::from_be_bytes(self.to_ext_bytes(1).try_into().unwrap())
//     }
// }

// impl From<&u16> for Integer{
//     fn from(value: &u16) -> Self {
//         Integer::from(&value.to_be_bytes()[..])
//     }
// }

// impl Into<u16> for &Integer{
//     fn into(self) -> u16 {
//         u16::from_be_bytes(self.to_ext_bytes(2).try_into().unwrap())
//     }
// }

// impl From<&u32> for Integer{
//     fn from(value: &u32) -> Self {
//         Integer::from(&value.to_be_bytes()[..])
//     }
// }

// impl Into<u32> for &Integer{
//     fn into(self) -> u32 {
//         u32::from_be_bytes(self.to_ext_bytes(4).try_into().unwrap())
//     }
// }

// impl From<&u64> for Integer{
//     fn from(value: &u64) -> Self {
//         Integer::from(&value.to_be_bytes()[..])
//     }
// }

// impl Into<u64> for &Integer{
//     fn into(self) -> u64 {
//         u64::from_be_bytes(self.to_ext_bytes(8).try_into().unwrap())
//     }
// }

// impl From<&u128> for Integer{
//     fn from(value: &u128) -> Self {
//         Integer::from(&value.to_be_bytes()[..])
//     }
// }

// impl Into<u128> for &Integer{
//     fn into(self) -> u128 {
//         u128::from_be_bytes(self.to_ext_bytes(16).try_into().unwrap())
//     }
// }

// impl From<&usize> for Integer {

//     fn from(value: &usize) -> Self {
//         Integer::from(&value.to_be_bytes()[..])
//     }

// }

// impl Into<usize> for &Integer {
    
//     fn into(self) -> usize {
//         usize::from_be_bytes(self.to_ext_bytes((usize::BITS/8) as usize).try_into().unwrap())
//     }

// }

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
