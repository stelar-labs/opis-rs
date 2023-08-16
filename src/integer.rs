mod addition;
mod and;
mod comparison;
mod debug;
mod display;
mod divider;
mod division;
mod exponentiation;
mod ext_gcd;
mod extension;
mod inversion;
mod lfsr;
mod multiplication;
mod not;
mod or;
mod remainder;
mod shifting;
mod sign;
mod string_conversion;
mod subtraction;
mod truncation;
mod type_conversion;
mod xor;
use crate::{Bit, Integer};
use std::error::Error;
mod mod_pow;

impl Default for Integer {
    fn default() -> Self {
        Integer::zero()
    }
}

impl Integer {
    
    pub fn modulo(&self, modulus: &Integer) -> Result<Self, Box<dyn Error>> {

        if self == &Integer::zero() {

            Ok(Integer::zero())

        } else if modulus == &Integer::zero() {

            Err("a/0 is undefined!")?

        } else {

            let mut result = (self % modulus)?;

            while result < Integer::zero() {
                result += result.clone()

            }

            Ok(result)

        }

    }

    pub fn neg_one() -> Self { Integer(vec![Bit::One, Bit::One]) }

    pub fn zero() -> Self { Integer(vec![Bit::Zero, Bit::Zero]) }

    pub fn one() -> Self { Integer(vec![Bit::Zero, Bit::One]) }

    pub fn two() -> Self { Integer(vec![Bit::Zero, Bit::One, Bit::Zero]) }

    pub fn three() -> Self { Integer(vec![Bit::Zero, Bit::One, Bit::One]) }

    pub fn bits(&self) -> Vec<Bit> { self.0.clone() }

}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_sub() {
        assert_eq!(Integer::three() - Integer::one(), Integer::two());
    }
    #[test]
    fn test_mul() {
        assert_eq!(Integer::two() * Integer::one(), Integer::two());
    }
    #[test]
    fn test_div() {
        assert_eq!((Integer::three() / Integer::two()).unwrap(), Integer::one());
    }
    #[test]
    fn test_rem() {
        assert_eq!((Integer::three() % Integer::two()).unwrap(), Integer::one());
    }
    #[test]
    fn test_shl() {
        assert_eq!(Integer::one() << &1, Integer::two());
    }
    #[test]
    fn test_shr() {
        assert_eq!(Integer::two() >> &1, Integer::one());
    }
    #[test]
    fn test_from_bin() {
        assert_eq!(Integer::from_bin("010").unwrap(), Integer::two());
    }
    #[test]
    fn test_to_bin() {
        assert_eq!(Integer::two().to_bin(), "010");
    }
    #[test]
    fn test_from_dec() {
        assert_eq!(Integer::from_dec("3").unwrap(), Integer::three());
    }
    #[test]
    fn test_to_dec() {
        assert_eq!(Integer::three().to_dec(), "3");
    }
    #[test]
    fn test_from_hex() {
        assert_eq!(Integer::from_hex("01").unwrap(), Integer::one());
    }
    #[test]
    fn test_to_hex() {
        assert_eq!(Integer::one().to_hex(), "01");
    }
    #[test]
    fn test_from() {
        assert_eq!(Integer::from(&[2][..]), Integer::two());
    }
    #[test]
    fn test_into() {
        let int_bytes: Vec<u8> = Integer::two().into();
        assert_eq!(int_bytes, vec![2]);
    }
    #[test]
    fn text_exp() {
        assert_eq!(
            Integer::two().pow(&Integer::two()).unwrap(),
            Integer(vec![Bit::Zero, Bit::One, Bit::Zero, Bit::Zero])
        )
    }
    #[test]
    fn test_lfsr() {
        assert_eq!(Integer::one().lfsr(1), Integer(vec![Bit::Zero ^ Bit::One, Bit::Zero]))
    }
    #[test]
    fn test_eq() {
        assert!(Integer::two() == Integer::two());
    }

    #[test]
    fn test_gt() {
        assert!(Integer::three() > Integer::one());
    }

    #[test]
    fn test_lt() {
        assert!(Integer::one() < Integer::two());
    }
}