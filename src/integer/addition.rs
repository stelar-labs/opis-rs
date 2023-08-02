
use crate::{Integer, Bit};
use std::ops::{Add, AddAssign};

impl Add for Integer {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        &self + &b
    }
}

impl Add for &Integer {

    type Output = Integer;

    fn add(self, b: Self) -> Integer {

        if self == &Integer::zero() {
            b.clone()
        } else if b == &Integer::zero() {
            self.clone()
        } else {
            let sum_bits = adder(&self.0, &b.0);
            let mut result = Integer(sum_bits);
            result.truncate();
            result
        }

    }
}

impl Add<&u8> for &Integer {
    type Output = Integer;
    fn add(self, b: &u8) -> Integer {
        let b_int: Integer = b.into();
        self + &b_int
    }
}

impl Add<&u16> for &Integer {
    type Output = Integer;
    fn add(self, b: &u16) -> Integer {
        let b_int: Integer = b.into();
        self + &b_int
    }
}

impl Add<&u32> for &Integer {
    type Output = Integer;
    fn add(self, b: &u32) -> Integer {
        let b_int: Integer = b.into();
        self + &b_int
    }
}

impl Add<&u64> for &Integer {
    type Output = Integer;
    fn add(self, b: &u64) -> Integer {
        let b_int: Integer = b.into();
        self + &b_int
    }
}

impl Add<&u128> for &Integer {
    type Output = Integer;
    fn add(self, b: &u128) -> Integer {
        let b_int: Integer = b.into();
        self + &b_int
    }
}

impl Add<&usize> for &Integer {
    type Output = Integer;
    fn add(self, b: &usize) -> Integer {
        let b_int: Integer = b.into();
        self + &b_int
    }
}

impl AddAssign for Integer {
    fn add_assign(&mut self, b: Self) {
        *self = &self.clone() + &b
    }
}

impl AddAssign<&Integer> for Integer {
    fn add_assign(&mut self, b: &Integer) {
        *self = &self.clone() + b
    }
}

impl AddAssign<&u8> for Integer {
    fn add_assign(&mut self, b: &u8) {
        let b_int: Integer = b.into();
        *self = &self.clone() + &b_int
    }
}

impl AddAssign<&u16> for Integer {
    fn add_assign(&mut self, b: &u16) {
        let b_int: Integer = b.into();
        *self = &self.clone() + &b_int
    }
}

impl AddAssign<&u32> for Integer {
    fn add_assign(&mut self, b: &u32) {
        let b_int: Integer = b.into();
        *self = &self.clone() + &b_int
    }
}

impl AddAssign<&u64> for Integer {
    fn add_assign(&mut self, b: &u64) {
        let b_int: Integer = b.into();
        *self = &self.clone() + &b_int
    }
}

impl AddAssign<&u128> for Integer {
    fn add_assign(&mut self, b: &u128) {
        let b_int: Integer = b.into();
        *self = &self.clone() + &b_int
    }
}

impl AddAssign<&usize> for Integer {
    fn add_assign(&mut self, b: &usize) {
        let b_int: Integer = b.into();
        *self = &self.clone() + &b_int
    }
}

pub fn adder(a: &[Bit], b: &[Bit]) -> Vec<Bit> {
    
    let precision = if a.len() > b.len() { a.len() } else { b.len() } + 1;

    let mut a_pos = a.len() - 1;

    let mut b_pos = b.len() - 1;

    let mut carry = Bit::Zero;

    (0..precision)
        .into_iter()
        .rev()
        .fold(
            vec![],
            |sum, _|
            {

                let a_bit = if a_pos == 0 { a[0] } else { let r = a[a_pos]; a_pos -= 1; r };
                
                let b_bit = if b_pos == 0 { b[0] } else { let r = b[b_pos]; b_pos -= 1; r };

                let (carry_1, sum_1) = carry + a_bit;

                let (carry_2, sum_2) = sum_1 + b_bit;

                carry = carry_1 ^ carry_2;

                vec![sum_2].into_iter().chain(sum.into_iter()).collect()

            }
        
        )

}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_int_plus_int() {
        assert_eq!(Integer::one() + Integer::one(), Integer::two());
    }

    #[test]
    fn test_int_plus_u8() {
        assert_eq!(&Integer::one() + &1_u8, Integer::two());
    }

}
