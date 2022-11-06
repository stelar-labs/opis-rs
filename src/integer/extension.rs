use crate::{Bit, Integer};

impl Integer {

    pub fn to_ext_bits(&self, length: usize) -> Vec<Bit> {

        if self.0.len() <= length {

            self.0[length - self.0.len()..].to_vec()

        } else {
            
            [vec![self.0[0]; length - self.0.len()], self.0.clone()].concat()

        }

    }

    pub fn to_ext_bytes(&self, length: usize) -> Vec<u8> {

        Integer(self.to_ext_bits(length * 8)).into()

    }

}
