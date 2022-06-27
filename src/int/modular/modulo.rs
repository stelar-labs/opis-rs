use crate::Int;
use std::error::Error;

impl Int {
    
    pub fn modulo(&self, modulus: &Int) -> Result<Self, Box<dyn Error>> {

        if self == &Int::zero() {

            Ok(Int::zero())

        } else if modulus == &Int::zero() {

            Err("a/0 is undefined!")?

        } else {

            let mut result = (self % modulus)?;

            while result < Int::zero() {

                result += result.clone()

            }

            Ok(result)

        }   

    }

}
