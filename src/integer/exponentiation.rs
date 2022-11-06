use crate::{Bit, Integer};

impl Integer {

    pub fn pow(&self, e: &Integer) -> Result<Integer, Box<dyn std::error::Error>> {
            
        if self == &Integer::zero() {
            
            Ok(Integer::zero())

        } else if e == &Integer::zero() {
            
            Ok(Integer::one())

        } else if e.0[0] == Bit::One {
            
            Err("Non Integer result for negative exponent!")?

        } else {
            
            Ok(
                e.0
                    .iter()
                    .skip(2)
                    .fold(
                        self.clone(),
                        |acc, x| {
                            let mut result = &acc * &acc;
                            if x == &Bit::One {
                                result *= self
                            }
                            result
                        }
                    )
            )

        }

    }

}
