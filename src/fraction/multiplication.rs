use crate::{Fraction, Integer};
use std::ops::{Mul, MulAssign};

impl Mul for Fraction {
    type Output = Fraction;
    fn mul(self, b: Self) -> Self::Output {
        &self * &b
    }
}

impl Mul for &Fraction {
    type Output = Fraction;
    fn mul(self, b: Self) -> Fraction {
        let mut result = Fraction(&self.0 * &b.0, &self.1 * &b.1);
        result.reduce();
        result
    }
}

impl Mul<&Integer> for &Fraction {
    type Output = Fraction;
    fn mul(self, b: &Integer) -> Fraction {
        let b_frac: Fraction = b.into();
        self * &b_frac
    }
}

impl Mul<&u8> for &Fraction {
    type Output = Fraction;
    fn mul(self, b: &u8) -> Fraction {
        let b_frac: Fraction = b.into();
        self * &b_frac
    }
}

impl Mul<&u16> for &Fraction {
    type Output = Fraction;
    fn mul(self, b: &u16) -> Fraction {
        let b_frac: Fraction = b.into();
        self * &b_frac
    }
}

impl Mul<&u32> for &Fraction {
    type Output = Fraction;
    fn mul(self, b: &u32) -> Fraction {
        let b_frac: Fraction = b.into();
        self * &b_frac
    }
}

impl Mul<&u64> for &Fraction {
    type Output = Fraction;
    fn mul(self, b: &u64) -> Fraction {
        let b_frac: Fraction = b.into();
        self * &b_frac
    }
}

impl Mul<&u128> for &Fraction {
    type Output = Fraction;
    fn mul(self, b: &u128) -> Fraction {
        let b_frac: Fraction = b.into();
        self * &b_frac
    }
}

impl Mul<&usize> for &Fraction {
    type Output = Fraction;
    fn mul(self, b: &usize) -> Fraction {
        let b_frac: Fraction = b.into();
        self * &b_frac
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, b: Self) {
        *self = &self.clone() * &b
    }
}

impl MulAssign<&Fraction> for Fraction {
    fn mul_assign(&mut self, b: &Fraction) {
        *self = &self.clone() * b
    }
}

impl MulAssign<&Integer> for Fraction {
    fn mul_assign(&mut self, b: &Integer) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() * &b_frac
    }
}

impl MulAssign<&u8> for Fraction {
    fn mul_assign(&mut self, b: &u8) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() * &b_frac
    }
}

impl MulAssign<&u16> for Fraction {
    fn mul_assign(&mut self, b: &u16) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() * &b_frac
    }
}

impl MulAssign<&u32> for Fraction {
    fn mul_assign(&mut self, b: &u32) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() * &b_frac
    }
}

impl MulAssign<&u64> for Fraction {
    fn mul_assign(&mut self, b: &u64) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() * &b_frac
    }
}

impl MulAssign<&u128> for Fraction {
    fn mul_assign(&mut self, b: &u128) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() * &b_frac
    }
}

impl MulAssign<&usize> for Fraction {
    fn mul_assign(&mut self, b: &usize) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() * &b_frac
    }
}
