use crate::Integer;
use std::ops::{Sub, SubAssign};

impl Sub for Integer {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        &self - &b
    }
}

impl Sub for &Integer {
    type Output = Integer;
    fn sub(self, b: Self) -> Integer {
        if b == &Integer::zero() {
            self.clone()
        } else {
            self + &b.inversion()
        }
    }
}

impl Sub<&u8> for &Integer {
    type Output = Integer;
    fn sub(self, b: &u8) -> Integer {
        let b_int: Integer = b.into();
        self - &b_int
    }
}

impl Sub<&u16> for &Integer {
    type Output = Integer;
    fn sub(self, b: &u16) -> Integer {
        let b_int: Integer = b.into();
        self - &b_int
    }
}

impl Sub<&u32> for &Integer {
    type Output = Integer;
    fn sub(self, b: &u32) -> Integer {
        let b_int: Integer = b.into();
        self - &b_int
    }
}

impl Sub<&u64> for &Integer {
    type Output = Integer;
    fn sub(self, b: &u64) -> Integer {
        let b_int: Integer = b.into();   
        self - &b_int
    }
}

impl Sub<&u128> for &Integer {
    type Output = Integer;
    fn sub(self, b: &u128) -> Integer {
        let b_int: Integer = b.into();
        self - &b_int
    }
}

impl Sub<&usize> for &Integer {
    type Output = Integer;
    fn sub(self, b: &usize) -> Integer {
        let b_int: Integer = b.into();
        self - &b_int
    }
}

impl SubAssign for Integer {
    fn sub_assign(&mut self, b: Self) {
        *self = &self.clone() - &b
    }
}

impl SubAssign<&Integer> for Integer {
    fn sub_assign(&mut self, b: &Self) {
        *self = &self.clone() - b
    }
}

impl SubAssign<&u8> for Integer {
    fn sub_assign(&mut self, b: &u8) {
        let b_int: Integer = b.into();
        *self = &self.clone() - &b_int
    }
}

impl SubAssign<&u16> for Integer {
    fn sub_assign(&mut self, b: &u16) {
        let b_int: Integer = b.into();
        *self = &self.clone() - &b_int
    }
}

impl SubAssign<&u32> for Integer {
    fn sub_assign(&mut self, b: &u32) {  
        let b_int: Integer = b.into();
        *self = &self.clone() - &b_int
    }
}

impl SubAssign<&u64> for Integer {
    fn sub_assign(&mut self, b: &u64) {
        let b_int: Integer = b.into();
        *self = &self.clone() - &b_int
    }
}

impl SubAssign<&u128> for Integer {
    fn sub_assign(&mut self, b: &u128) {
        let b_int: Integer = b.into();
        *self = &self.clone() - &b_int
    }
}

impl SubAssign<&usize> for Integer {
    fn sub_assign(&mut self, b: &usize) {
        let b_int: Integer = b.into();
        *self = &self.clone() - &b_int
    }
}
