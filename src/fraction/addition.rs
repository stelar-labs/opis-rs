use crate::{Fraction, Integer};
use std::ops::{Add, AddAssign};

impl Add for Fraction {
    type Output = Fraction;
    fn add(self, b: Self) -> Self::Output {
        &self + &b
    }
}

impl Add for &Fraction {
    type Output = Fraction;
    fn add(self, b: Self) -> Self::Output {
        if self == &Fraction::zero() {
            b.clone()
        } else if b == &Fraction::zero() {
            self.clone()
        } else {
            let mut result = Fraction((&self.0 * &b.1) + (&b.0 * &self.1), &self.1 * &b.1);
            result.reduce();
            result
        }
    }
}

impl Add<&Integer> for &Fraction {

    type Output = Fraction;
    
    fn add(self, b: &Integer) -> Fraction {
        let b_frac: Fraction = b.into();
        self + &b_frac
    }

}

impl Add<&u8> for &Fraction {

    type Output = Fraction;

    fn add(self, b: &u8) -> Fraction {
        let b_frac: Fraction = b.into();
        self + &b_frac
    }

}

impl Add<&u16> for &Fraction {

    type Output = Fraction;

    fn add(self, b: &u16) -> Fraction {

        let b_frac: Fraction = b.into();

        self + &b_frac

    }
}

impl Add<&u32> for &Fraction {
    type Output = Fraction;
    fn add(self, b: &u32) -> Fraction {
        let b_frac: Fraction = b.into();
        self + &b_frac
    }
}

impl Add<&u64> for &Fraction {
    type Output = Fraction;
    fn add(self, b: &u64) -> Fraction {
        let b_frac: Fraction = b.into();
        self + &b_frac
    }
}

impl Add<&u128> for &Fraction {
    type Output = Fraction;
    fn add(self, b: &u128) -> Fraction {
        let b_frac: Fraction = b.into();
        self + &b_frac
    }
}

impl Add<&usize> for &Fraction {
    type Output = Fraction;
    fn add(self, b: &usize) -> Fraction {
        let b_frac: Fraction = b.into();
        self + &b_frac
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, b: Self) {
        *self = &self.clone() + &b
    }
}

impl AddAssign<&Fraction> for Fraction {
    fn add_assign(&mut self, b: &Fraction) {
        *self = &self.clone() + b
    }
}

impl AddAssign<&Integer> for Fraction {
    fn add_assign(&mut self, b: &Integer) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() + &b_frac
    }
}

impl AddAssign<&u8> for Fraction {
    fn add_assign(&mut self, b: &u8) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() + &b_frac
    }
}

impl AddAssign<&u16> for Fraction {
    fn add_assign(&mut self, b: &u16) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() + &b_frac
    }
}

impl AddAssign<&u32> for Fraction {
    fn add_assign(&mut self, b: &u32) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() + &b_frac
    }
}

impl AddAssign<&u64> for Fraction {
    fn add_assign(&mut self, b: &u64) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() + &b_frac
    }
}

impl AddAssign<&u128> for Fraction {
    fn add_assign(&mut self, b: &u128) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() + &b_frac
    }
}

impl AddAssign<&usize> for Fraction {
    fn add_assign(&mut self, b: &usize) {
        let b_frac: Fraction = b.into();
        *self = &self.clone() + &b_frac
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fra_add_0() {

        let a = Fraction::try_from("1/2").unwrap();

        let r = Fraction::try_from("1/1").unwrap();

        assert_eq!(&a + &a, r)

    }

    #[test]
    fn test_fra_add_1() {

        let a = Fraction::try_from("-2/1").unwrap();

        let b = Fraction::try_from("1/1").unwrap();

        let r = Fraction::try_from("-1/1").unwrap();

        assert_eq!(&a + &b, r)

    }

}

