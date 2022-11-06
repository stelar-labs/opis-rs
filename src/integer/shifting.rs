use crate::{Integer, Bit};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

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
    Integer([a.0.to_vec(), vec![Bit::Zero; *shifts]].concat())
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

    if shifts <= &(a.0.len() - 2) {

        Integer(a.0[0..a.0.len() - shifts].to_vec())

    } else {

        Integer(vec![a.0[0];2])

    }
}
