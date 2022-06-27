use crate::{Int, Bit};
use std::error::Error;

impl Int {

    pub fn get_left_bit(&self, index: usize, precision: usize) -> Result<Bit, Box<dyn Error>> {

        if index < precision {

            if self.bits.len() >= precision {

                Ok(self.bits[index])
            
            } else {

                let diff = precision - self.bits.len();

                if index > diff {
                
                    Ok(self.bits[index - diff])
                
                } else {
                    
                    Ok(self.bits[0])
                
                }

            }

        } else {

            Err("Internal error!")?
        
        }

    }

    pub fn get_right_bit(&self, index: usize, precision: usize) -> Result<Bit, Box<dyn Error>> {

        if index < precision {

            if index < self.bits.len() {

                let start = self.bits.len() - 1;

                let right_index = start - index;

                let right_bit = self.bits[right_index];

                Ok(right_bit)

            } else {

                let right_bit = self.bits[0];

                Ok(right_bit)

            }

        } else {

            Err("Internal error!")?
        
        }

    }

}
