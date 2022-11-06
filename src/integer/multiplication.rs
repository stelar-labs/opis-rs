
use crate::{Integer, Bit};
use std::ops::{Mul, MulAssign};

impl Mul for &Integer {
    type Output = Integer;
    fn mul(self, b: Self) -> Integer {
        
        let precision = if self.0.len() > b.0.len() {
            self.0.len() * 2
        } else {
            b.0.len() * 2
        };

        (0..precision)
            .into_iter()
            .fold(Integer::zero(), |acc, x| {
                
                let b_bit =
                    if x > b.0.len() - 1 {
                        b.0[0]
                    } else {
                        b.0[b.0.len() - x - 1]
                    };

                if b_bit == Bit::One {
                    acc + (self << &x)
                } else {
                    acc
                }

            })

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
