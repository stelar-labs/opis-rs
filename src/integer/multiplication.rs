
use crate::{Integer, Bit};
use std::ops::{Mul, MulAssign};

use super::addition::adder;

impl Mul for &Integer {

    type Output = Integer;
    
    fn mul(self, b: Self) -> Integer {

        if self == &Integer::zero() || b == &Integer::zero() {
            Integer::zero()
        } else if self == &Integer::one() {
            b.clone()
        } else if b == &Integer::one() {
            self.clone()
        } else if self == &Integer::neg_one() {
            b.inversion()
        } else if b == &Integer::neg_one() {
            self.inversion()
        } else {

            let precision = if self.0.len() > b.0.len() { self.0.len() } else { b.0.len() } * 2;

            let extended_a: Vec<Bit> =

                vec![self.0[0]; precision - self.0.len()]
                    .into_iter()
                    .chain(
                        self
                            .0
                            .clone()
                            .into_iter()
                    )
                    .collect();

            let extended_b: Vec<Bit> =

                vec![b.0[0]; precision - b.0.len()]
                    .into_iter()
                    .chain(
                        b
                            .0
                            .clone()
                            .into_iter()
                    )
                    .collect();

            let product_bits =

                extended_b
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(
                        vec![Bit::Zero, Bit::Zero],
                        |acc, (i, x)|
                        {

                            if x == &Bit::One {

                                let shifted_a: Vec<Bit> =
                                    
                                    extended_a
                                        .clone()
                                        .into_iter()
                                        .chain(
                                            vec![Bit::Zero; i]
                                                .into_iter()
                                        )
                                        .collect();

                                adder(&acc, &shifted_a)

                            } else {
                                
                                acc

                            }

                        }
                    );

            let mut result = Integer(
                product_bits
                    .iter()
                    .rev()
                    .take(precision)
                    .rev()
                    .cloned()
                    .collect()
            );

            result.truncate();

            result

        }

    }

}

impl Mul for Integer {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        &self * &b
    }
}

impl Mul<&u8> for &Integer {
    type Output = Integer;
    fn mul(self, b: &u8) -> Integer {
        let b_int: Integer = b.into();
        self * &b_int
    }
}

impl Mul<&u16> for &Integer {
    type Output = Integer;
    fn mul(self, b: &u16) -> Integer {
        let b_int: Integer = b.into();
        self * &b_int
    }
}

impl Mul<&u32> for &Integer {
    type Output = Integer;
    fn mul(self, b: &u32) -> Integer {
        let b_int: Integer = b.into();
        self * &b_int
    }
}

impl Mul<&u64> for &Integer {
    type Output = Integer;
    fn mul(self, b: &u64) -> Integer {
        let b_int: Integer = b.into();
        self * &b_int
    }
}

impl Mul<&u128> for &Integer {
    type Output = Integer;
    fn mul(self, b: &u128) -> Integer {
        let b_int: Integer = b.into();
        self * &b_int
    }
}

impl Mul<&usize> for &Integer {
    type Output = Integer;
    fn mul(self, b: &usize) -> Integer {
        let b_int: Integer = b.into();
        self * &b_int
    }
}

impl MulAssign for Integer {
    fn mul_assign(&mut self, b: Self) {
        *self = self.clone() * b
    }
}

impl MulAssign<&Integer> for Integer {
    fn mul_assign(&mut self, b: &Integer) {
        *self = &self.clone() * b
    }
}

impl MulAssign<&u8> for Integer {
    fn mul_assign(&mut self, b: &u8) {
        let b_int: Integer = b.into();
        *self = self.clone() * b_int
    }
}

impl MulAssign<&u16> for Integer {
    fn mul_assign(&mut self, b: &u16) {
        let b_int: Integer = b.into();
        *self = self.clone() * b_int
    }
}

impl MulAssign<&u32> for Integer {
    fn mul_assign(&mut self, b: &u32) {
        let b_int: Integer = b.into();
        *self = self.clone() * b_int
    }
}

impl MulAssign<&u64> for Integer {
    fn mul_assign(&mut self, b: &u64) {
        let b_int: Integer = b.into();
        *self = self.clone() * b_int
    }
}

impl MulAssign<&u128> for Integer {
    fn mul_assign(&mut self, b: &u128) {
        let b_int: Integer = b.into();
        *self = self.clone() * b_int
    }
}

impl MulAssign<&usize> for Integer {
    fn mul_assign(&mut self, b: &usize) {
        let b_int: Integer = b.into();
        *self = self.clone() * b_int
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_int_mul_0() {

        let a = Integer::from_dec("-6").unwrap();

        let b = Integer::from_dec("-1").unwrap();

        let c = Integer::from_dec("6").unwrap();

        assert_eq!(a * b, c);
        
    }

}