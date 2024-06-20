use crate::{Integer, Bit};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use super::digit::Digit;

impl Shl<&usize> for Integer {
    type Output = Integer;
    fn shl(self, shifts: &usize) -> Integer {
        shift_left(&self, shifts)
    }
}

impl Shl<&Integer> for Integer {
    type Output = Integer;
    fn shl(self, shifts: &Integer) -> Integer {
        shift_left(&self, &shifts.into())
    }
}

impl Shl<&usize> for &Integer {
    type Output = Integer;
    fn shl(self, shifts: &usize) -> Integer {
        shift_left(&self, shifts)
    }
}

impl Shl<&Integer> for &Integer {
    type Output = Integer;
    fn shl(self, shifts: &Integer) -> Integer {
        shift_left(self, &shifts.into())
    }
}

impl ShlAssign<&usize> for Integer {
    fn shl_assign(&mut self, shifts: &usize) {
        * self = shift_left(self, shifts)
    }
}

impl ShlAssign<&Integer> for Integer {
    fn shl_assign(&mut self, shifts: &Integer) {
        * self = shift_left(self, &shifts.into())
    }
}

fn shift_left(a: &Integer, shifts: &usize) -> Integer {
    if a.digits.len() == 1 {
        Integer { digits: vec![a.digits[0] << shifts], }
    } else {

        let total_bits = std::mem::size_of::<Digit>() * 8;
        let digit_shifts = shifts / total_bits;
        let msb_set = a.most_significant_bit(Some(total_bits));

        if digit_shifts >= a.digits.len() {
            return if msb_set {
                Integer { digits: vec![Digit::MAX] }
            } else {
                Integer { digits: vec![0] }
            };
        }

        let bit_shifts_to_the_left = shifts % total_bits;
        let bit_shifts_to_the_right = total_bits - bit_shifts_to_the_left;

        let mut digits = vec![];

        let mut carry_digit = if msb_set {
            Digit::MAX >> bit_shifts_to_the_right
        } else {
            Digit::MIN
        };

        for i in (digit_shifts..a.digits.len()).rev() {
            let current_digit = a.digits[i];
            let carry_bits = current_digit >> bit_shifts_to_the_right;
            let shifted_bits = current_digit << bit_shifts_to_the_left;
            let combined_digit = shifted_bits | carry_digit;

            carry_digit = carry_bits;
            digits.push(combined_digit);
        }

        digits.reverse();

        Integer { digits }

    }
}

impl Shr<&usize> for Integer {
    type Output = Integer;
    fn shr(self, shifts: &usize) -> Integer {
        shift_right(&self, shifts)
    }
}

impl Shr<&Integer> for Integer {
    type Output = Integer;
    fn shr(self, shifts: &Integer) -> Integer {
        shift_right(&self, &shifts.into())
    }
}

impl Shr<&usize> for &Integer {
    type Output = Integer;
    fn shr(self, shifts: &usize) -> Integer {
        shift_right(&self, shifts)
    }
}

impl Shr<&Integer> for &Integer {
    type Output = Integer;
    fn shr(self, shifts: &Integer) -> Integer {
        shift_right(&self, &shifts.into())
    }
}

impl ShrAssign<&usize> for Integer {
    fn shr_assign(&mut self, shifts: &usize) {
        *self = shift_right(self, shifts)
    }
}

impl ShrAssign<&Integer> for Integer {
    fn shr_assign(&mut self, shifts: &Integer) {
        *self = shift_right(self, &shifts.into())
    }
}

fn shift_right(a: &Integer, shifts: &usize) -> Integer {

    let total_bits = std::mem::size_of::<Digit>() * 8;
    let digit_shifts = shifts / total_bits;

    let mut digits = vec![Digit::MIN; digit_shifts];

    let bit_shifts_to_the_right = shifts % total_bits;
    let bit_shifts_to_the_left = total_bits - bit_shifts_to_the_right;

    let mut carry_digit = Digit::MIN;

    for digit in &a.digits {

        let carry_bits = digit << bit_shifts_to_the_left;

        let shifted_bits = digit >> bit_shifts_to_the_right;

        let combined_digit = shifted_bits | carry_digit;

        carry_digit = carry_bits;
        digits.push(combined_digit);

    }

    let msb_set = a.most_significant_bit(Some(total_bits));

    if a.count_extended_bits(msb_set) > bit_shifts_to_the_right {

        let carry_ext = if msb_set {
            Digit::MAX >> bit_shifts_to_the_right
        } else {
            Digit::MIN
        };

        digits.push(carry_digit | carry_ext);

    }

    Integer { digits }
}
