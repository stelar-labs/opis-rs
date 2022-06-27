use crate::{ Bit, Int };
use std::error::Error;

impl Int {

    pub fn pow(&self, e: &Int) -> Result<Int, Box<dyn Error>> {
        
        if self == &Int::zero() {
            
            Ok(Int::zero())
        
        } else if e == &Int::zero() {
            
            Ok(Int::one())
        
        } else if e.bits[0] == Bit::One {
            
            Err("Non Integer result for negative exponent!")?

        } else {

            let mut result = self.clone();

            e.bits
                .iter()
                .skip(2)
                .for_each(|x| {

                    result = &result * &result;

                    if x == &Bit::One {

                        result = &result * self

                    }
                    
                }
                
            );

            Ok(result)
            
        }

    }

}
