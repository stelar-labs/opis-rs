use std::error::Error;

use crate::Integer;


impl Integer {
    
    pub fn mod_pow(&self, exponent: &Self, modulus: &Self) -> Result<Integer, Box<dyn Error>> {

        if modulus == &Integer::one() {

            Ok(Integer::zero())

        } else {

            let mut result = Integer::one();
            
            let mut base = (self % modulus)?;
            
            let mut exp = exponent.clone();
            
            while exp > Integer::zero() {
                
                if (&exp % &Integer::two())? == Integer::one() {
                    
                    result = (&(&result * &base) % modulus)?;
                
                }

                base = (&(&base * &base) % modulus)?;

                exp = (&exp / &Integer::two())?;

            }
            
            Ok(result)

        }
   
    }

}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_base_11_exp_13_mod_19() {

        let base = Integer::from_dec("11").unwrap();

        let exponent = Integer::from_dec("13").unwrap();

        let modulus = Integer::from_dec("19").unwrap();

        assert_eq!(
            base.mod_pow(&exponent, &modulus).unwrap(),
            base
        );

    }

    #[test]
    fn test_base_7_exp_2_mod_13() {

        let base = Integer::from_dec("7").unwrap();

        let exponent = Integer::from_dec("2").unwrap();

        let modulus = Integer::from_dec("13").unwrap();

        assert_eq!(
            base.mod_pow(&exponent, &modulus).unwrap(),
            Integer::from_dec("10").unwrap()
        );

    }

}

