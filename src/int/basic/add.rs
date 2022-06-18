use crate::{ Bit, Int };
use std::ops::{Add, AddAssign};

impl Add for Int {
    
    type Output = Self;
    
    fn add(self, b: Self) -> Self {
        add(&self, &b)
    }

}

impl Add for &Int {

    type Output = Int;
    
    fn add(self, b: Self) -> Int {
        add(self, b)
    }

}

impl AddAssign for Int {
    
    fn add_assign(&mut self, b: Self) {
        *self = add(&self, &b)
    }

}

fn add(a: &Int, b: &Int) -> Int {

    let precision =

        if a.bits.len() > b.bits.len() {

            a.bits.len() + 1
        
        } else {

            b.bits.len() + 1
        
        };

    let mut bits = vec![Bit::Zero; precision];

    let mut carry = Bit::Zero;

    for index in 0..precision {

        let a_bit = a.get_right_bit(index, precision).unwrap();

        let (carry_1, sum_1) = carry + a_bit;

        let b_bit = b.get_right_bit(index, precision).unwrap();

        let (carry_2, sum_2) = b_bit + sum_1;

        carry = carry_1 ^ carry_2;

        let bit_index = precision - 1 - index;

        bits[bit_index] = sum_2

    }

    while bits.len() > 2 && bits[0] == bits[1] {

        bits.remove(0);

    }

    Int { bits }

}
