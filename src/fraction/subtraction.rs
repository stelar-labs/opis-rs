use crate::{Fraction, Integer};
use std::ops::{Sub, SubAssign};

impl Sub for Fraction {
    type Output = Fraction;
    fn sub(self, b: Self) -> Fraction {
        &self - &b
    }
}

impl Sub for &Fraction {
    type Output = Fraction;
    fn sub(self, b: Self) -> Fraction {
        let mut result = Fraction((&self.0 * &b.1) - (&b.0 * &self.1), &self.1 * &b.1);
        result.reduce();
        result
    }
}

impl Sub<&Integer> for &Fraction {
    type Output = Fraction;
    fn sub(self, b: &Integer) -> Fraction {
        let b_frac: Fraction = b.into();
        self - &b_frac
    }
}

impl Sub<&u8> for &Fraction {
    type Output = Fraction;
    fn sub(self, b: &u8) -> Fraction {
        let b_frac: Fraction = b.into();
        self - &b_frac
    }
}

impl Sub<&u16> for &Fraction {
    type Output = Fraction;
    fn sub(self, b: &u16) -> Fraction {
        let b_frac: Fraction = b.into();
        self - &b_frac
    }
}

impl Sub<&u32> for &Fraction {
    type Output = Fraction;
    fn sub(self, b: &u32) -> Fraction {
        let b_frac: Fraction = b.into();
        self - &b_frac
    }
}

impl Sub<&u64> for &Fraction {
    type Output = Fraction;
    fn sub(self, b: &u64) -> Fraction {
        let b_frac: Fraction = b.into();
        self - &b_frac
    }
}

impl Sub<&u128> for &Fraction {
    type Output = Fraction;
    fn sub(self, b: &u128) -> Fraction {
        let b_frac: Fraction = b.into();
        self - &b_frac
    }
}

impl Sub<&usize> for &Fraction {
    type Output = Fraction;
    fn sub(self, b: &usize) -> Fraction {
        let b_frac: Fraction = b.into();
        self - &b_frac
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, b: Self) {
        *self = &self.clone() - &b
    }
}

impl SubAssign<&Fraction> for Fraction {
    fn sub_assign(&mut self, b: &Fraction) {
        *self = &self.clone() - b
    }
}

impl SubAssign<&Integer> for Fraction {
    fn sub_assign(&mut self, b: &Integer) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() - &b_frac
    }
}

impl SubAssign<&u8> for Fraction {
    fn sub_assign(&mut self, b: &u8) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() - &b_frac
    }
}

impl SubAssign<&u16> for Fraction {
    fn sub_assign(&mut self, b: &u16) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() - &b_frac
    }
}

impl SubAssign<&u32> for Fraction {
    fn sub_assign(&mut self, b: &u32) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() - &b_frac
    }
}

impl SubAssign<&u64> for Fraction {
    fn sub_assign(&mut self, b: &u64) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() - &b_frac
    }
}

impl SubAssign<&u128> for Fraction {
    fn sub_assign(&mut self, b: &u128) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() - &b_frac
    }
}

impl SubAssign<&usize> for Fraction {
    fn sub_assign(&mut self, b: &usize) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() - &b_frac
    }
}
