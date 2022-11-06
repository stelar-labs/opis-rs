use crate::{Integer, Bit};

impl From<&[Bit]> for Integer{

    fn from(bits: &[Bit]) -> Self {

        if bits.is_empty() {

            Integer::zero()

        } else if bits.len() == 1 {

            Integer(vec![Bit::Zero, bits[0]])

        } else {

            Integer(bits.to_vec())

        }
    }
}

impl From<Vec<u8>> for Integer{
    fn from(value: Vec<u8>) -> Self {
        Integer::from(&value[..])
    }
}

impl From<&[u8]> for Integer{
    fn from(bytes: &[u8]) -> Self {
        Integer::from_bin(
            &bytes
                .iter()
                .fold(String::new(), |acc, x| format!("{}{:08b}", acc, x))
        ).unwrap()
    }
}

impl Into<Vec<u8>> for Integer{
    fn into(self) -> Vec<u8> {
        bits_to_bytes(&self.0)
    }
}

impl Into<Vec<u8>> for &Integer{
    fn into(self) -> Vec<u8> {
        bits_to_bytes(&self.0)
    }
}

fn bits_to_bytes(bits: &[Bit]) -> Vec<u8> {

    let r = bits.len() % 8;
    
    let (bytes, _) = bits
        .iter()
        .fold((vec![], vec![bits[0]; 8 - r]), |(mut bytes, mut byte), x| {
            
            byte.push(*x);
            
            if byte.len() == 8 {

                let byte_string: String = byte
                    .iter()
                    .map(|y| {
                        match y {
                            Bit::One => '1',
                            Bit::Zero => '0'
                        }
                    })
                    .collect();

                bytes.push(u8::from_str_radix(&byte_string, 2).unwrap());

                byte.clear()
                
            }

            (bytes, byte)
            
        });
        
    bytes

}

impl From<&u8> for Integer{
    fn from(value: &u8) -> Self {
        Integer::from(&value.to_le_bytes()[..])
    }
}

impl Into<u8> for &Integer{
    fn into(self) -> u8 {
        u8::from_le_bytes(self.to_ext_bytes(1).try_into().unwrap())
    }
}

impl From<&u16> for Integer{
    fn from(value: &u16) -> Self {
        Integer::from(&value.to_le_bytes()[..])
    }
}

impl Into<u16> for &Integer{
    fn into(self) -> u16 {
        u16::from_le_bytes(self.to_ext_bytes(2).try_into().unwrap())
    }
}

impl From<&u32> for Integer{
    fn from(value: &u32) -> Self {
        Integer::from(&value.to_le_bytes()[..])
    }
}

impl Into<u32> for &Integer{
    fn into(self) -> u32 {
        u32::from_le_bytes(self.to_ext_bytes(4).try_into().unwrap())
    }
}

impl From<&u64> for Integer{
    fn from(value: &u64) -> Self {
        Integer::from(&value.to_le_bytes()[..])
    }
}

impl Into<u64> for &Integer{
    fn into(self) -> u64 {
        u64::from_le_bytes(self.to_ext_bytes(8).try_into().unwrap())
    }
}

impl From<&u128> for Integer{
    fn from(value: &u128) -> Self {
        Integer::from(&value.to_le_bytes()[..])
    }
}

impl Into<u128> for &Integer{
    fn into(self) -> u128 {
        u128::from_le_bytes(self.to_ext_bytes(16).try_into().unwrap())
    }
}

impl From<&usize> for Integer{
    fn from(value: &usize) -> Self {
        Integer::from(&value.to_le_bytes()[..])
    }
}

impl Into<usize> for &Integer{
    fn into(self) -> usize {
        usize::from_le_bytes(self.to_ext_bytes((usize::BITS/8) as usize).try_into().unwrap())
    }
}
