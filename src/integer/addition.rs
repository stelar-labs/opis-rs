
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

        let (w,x,y,z) = if self.0.len() > b.0.len() {

            let d = self.0.len() - b.0.len();

            (vec![vec![self.0[0]; 2], self.0[0..d].to_vec()].concat(), &b.0[0], &self.0[d..], &b.0)

        } else {

            let d = b.0.len() - self.0.len();

            (vec![vec![b.0[0]; 2], b.0[0..d].to_vec()].concat(), &self.0[0], &b.0[d..], &self.0)

        };
        
        let (mut b2, b1_c) = (0..y.len())
            .into_iter()
            .rev()
            .fold((vec![], Bit::Zero), |(mut bits, c), o| {

                let (c1, s1) = &c + &y[o];

                let (c2, s2) = z[o] + s1;

                bits.push(s2);

                (bits, c1 ^ c2)

            });

        b2.reverse();

        let (mut b1, _) = (0..w.len())
            .into_iter()
            .rev()
            .fold((vec![], b1_c), |(mut bits, c), o| {

                let (c1, s1) = c + w[o];

                let (c2, s2) = s1 + x;

                bits.push(s2);

                (bits, c1 ^ c2)

            });

        b1.reverse();

        let sum_bits = vec![b1, b2].concat();

        let mut result = Integer(sum_bits);

        result.clean();

        result


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
