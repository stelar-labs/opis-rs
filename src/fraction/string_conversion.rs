
use crate::{Fraction, Integer};

impl TryFrom<&str> for Fraction {

    fn try_from(value: &str) -> Result<Fraction, Box<dyn std::error::Error>> {
        
        let split_value: Vec<&str> = value.split("/").collect();

        match split_value.get(0) {

            Some(&a_str) => {

                let a = {
                    
                    let e_split: Vec<&str> = a_str.split("e").collect();

                    if e_split.is_empty() {

                        Fraction::zero()

                    } else {

                        if e_split.len() == 1 {

                            let e = match e_split[0].find('.') {
                                Some(e_i) => (&(e_split[0].len() - e_i - 1)).into(),
                                None => Integer::zero(),
                            };

                            let mantissa_str = e_split[0].replace(".", "");

                            Fraction(
                                Integer::try_from(&mantissa_str[..])?,
                                Integer::ten().pow(&e)?
                            )

                        } else {
                            
                            let mantissa = Fraction::try_from(e_split[0])?;

                            let exponent = Integer::try_from(e_split[1])?;

                            mantissa * match exponent.0[0] {
                                crate::Bit::One => {
                                    Fraction(Integer::one(), Integer::ten().pow(&exponent.inversion())?)
                                },
                                crate::Bit::Zero => {
                                    Fraction(Integer::ten().pow(&exponent)?, Integer::one())
                                },
                            }

                        }

                    }
                    
                };

                if a == Fraction::zero() {

                    Ok(Fraction::zero())

                } else {

                    match split_value.get(1) {

                        Some(&b_str) => {
                            
                            let b = Fraction::try_from(b_str)?;

                            if b == Fraction::zero() {

                                Ok(Fraction::zero())

                            } else {

                                a / b
                                
                            }
                            
                        },
                        
                        None => Ok(a)

                    }

                }

            },

            None => Ok(Fraction::default())

        }

    }

    type Error = Box<dyn std::error::Error>;

}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_try_from_str_0() {
        assert_eq!(Fraction::try_from("1/2").unwrap(), Fraction(Integer::one(), Integer::two()));
    }

    #[test]
    fn test_try_from_str_1() {
        assert_eq!(
            Fraction::try_from("1e-1").unwrap(),
            Fraction(Integer::one(), Integer::ten())
        );
    }

    #[test]
    fn test_try_from_str_2() {
        assert_eq!(
            Fraction::try_from("3e-1").unwrap(),
            Fraction(Integer::three(),Integer::ten())
        );
    }

    #[test]
    fn test_try_from_str_3() {
        assert_eq!(
            Fraction::try_from("0.3e-2").unwrap(),
            Fraction(Integer::three(),Integer::ten().pow(&Integer::three()).unwrap())
        );
    }

}