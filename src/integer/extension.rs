use crate::{Bit, Integer};
use crate::integer::type_conversion::bits_to_bytes;

impl Integer {

    pub fn to_ext_bits(&self, length: usize) -> Vec<Bit> {

        if self.0.len() <= length {

            [vec![self.0[0]; length - self.0.len()], self.0.clone()].concat()

        } else {

            self.0[length - self.0.len()..].to_vec()

        }

    }

    pub fn to_ext_bytes(&self, length: usize) -> Vec<u8> {

        let ext_bits = self.to_ext_bits(length * 8);

        bits_to_bytes(&ext_bits)

    }

}
