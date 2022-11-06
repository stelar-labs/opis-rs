
use crate::{Fraction, Integer};

impl TryFrom<&str> for Fraction {

    fn try_from(value: &str) -> Result<Fraction, Box<dyn std::error::Error>> {
        
        let split_value: Vec<&str> = value.split("/").collect();

        if split_value.len() == 2 {

            Ok(Fraction(Integer::from_dec(split_value[0])?, Integer::from_dec(split_value[1])?))

        } else {

            Err("Improper fraction format!")?

        }

    }

    type Error = Box<dyn std::error::Error>;

}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_try_from_str() {
        assert_eq!(Fraction::try_from("1/2").unwrap(), Fraction(Integer::one(), Integer::two()));
    }

}