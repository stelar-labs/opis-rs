use crate::Int;
use std::error::Error;

impl Int {
    
    pub fn from_u8(integer: &u8) -> Result<Self, Box<dyn Error>> {

        let mut bytes = integer.to_be_bytes().to_vec();

        bytes = [vec![0], bytes].concat();

        Int::from_bytes(&bytes)

    }

    pub fn to_u8(&self) -> Result<u8, Box<dyn Error>> {

        let bytes = self.to_bytes();

        if bytes.len() == 1 {

            let integer = u8::from_be_bytes(bytes.try_into().unwrap());

            Ok(integer)

        } else {

            Err("Internal error!")?
        
        }
    }

    pub fn from_u16(integer: &u16) -> Result<Self, Box<dyn Error>> {

        let mut bytes = integer.to_be_bytes().to_vec();

        bytes = [vec![0], bytes].concat();

        Int::from_bytes(&bytes)

    }

    pub fn to_u16(&self) -> Result<u16, Box<dyn Error>> {

        let bytes = self.to_bytes();

        if bytes.len() == 2 {

            let integer = u16::from_be_bytes(bytes.try_into().unwrap());

            Ok(integer)

        } else {

            Err("Internal error!")?
        
        }
    }

    pub fn from_u32(integer: &u32) -> Result<Self, Box<dyn Error>> {

        let mut bytes = integer.to_be_bytes().to_vec();

        bytes = [vec![0], bytes].concat();

        Int::from_bytes(&bytes)

    }

    pub fn to_u32(&self) -> Result<u32, Box<dyn Error>> {

        let bytes = self.to_bytes();

        if bytes.len() == 4 {

            let integer = u32::from_be_bytes(bytes.try_into().unwrap());

            Ok(integer)

        } else {

            Err("Internal error!")?
        
        }
    }

    pub fn from_u64(integer: &u64) -> Result<Self, Box<dyn Error>> {

        let mut bytes = integer.to_be_bytes().to_vec();

        bytes = [vec![0], bytes].concat();

        Int::from_bytes(&bytes)

    }

    pub fn to_u64(&self) -> Result<u64, Box<dyn Error>> {

        let bytes = self.to_bytes();

        if bytes.len() == 8 {

            let integer = u64::from_be_bytes(bytes.try_into().unwrap());

            Ok(integer)

        } else {

            Err("Internal error!")?
        
        }

    }

}
