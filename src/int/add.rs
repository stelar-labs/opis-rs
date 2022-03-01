use std::ops::{Add, AddAssign};
use std::cmp::Ordering;

use crate::Bit;
use crate::Int;

use crate::int::cmp::comparator;
use crate::int::sub::subtractor;


impl Add for Int {
    
    type Output = Self;
    
    fn add(self, other: Self) -> Self {

        match (self.sign, other.sign) {

            (false, false) => Int {
                magnitude: adder(self.magnitude, other.magnitude),
                sign: false
            },

            (true, true) => Int {
                magnitude: adder(self.magnitude, other.magnitude),
                sign: true
            },

            (true, false) => {
                
                match comparator(&self.magnitude, &other.magnitude) {
                    
                    Ordering::Equal => Int::zero(),
                    
                    Ordering::Greater => Int {
                        magnitude: subtractor(self.magnitude, other.magnitude),
                        sign: true
                    },
                    
                    Ordering::Less => Int {
                        magnitude: subtractor(other.magnitude, self.magnitude),
                        sign: false
                    }

                }

            },

            (false, true) => {
                
                match comparator(&self.magnitude, &other.magnitude) {
                    
                    Ordering::Equal => Int::zero(),
                    
                    Ordering::Greater => Int {
                        magnitude: subtractor(self.magnitude, other.magnitude),
                        sign: false
                    },
                    
                    Ordering::Less => Int {
                        magnitude: subtractor(other.magnitude, self.magnitude),
                        sign: true
                    }

                }

            }

        }

    }

}

impl Add for &Int {

    type Output = Int;
    
    fn add(self, b: Self) -> Int {
        self.clone() + b.clone()
    }

}

impl AddAssign for Int {
    
    fn add_assign(&mut self, b: Self) {
        *self = self.clone() + b
    }

}

pub fn adder(mut a: Vec<Bit>, mut b: Vec<Bit>) -> Vec<Bit> {

    let mut res: Vec<Bit> = Vec::new();
    
    let mut carry: Bit = Bit::Zero;

    while !a.is_empty() || !b.is_empty() {

        let a_bit: Bit =
            match a.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        let b_bit: Bit =
            match b.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        match (carry, a_bit, b_bit) {
            
            (Bit::One, Bit::One, Bit::One) => {
                res.push(Bit::One);
                carry = Bit::One
            },
            
            (Bit::Zero, Bit::One, Bit::One) => {
                res.push(Bit::Zero);
                carry = Bit::One
            },
            
            (Bit::One, Bit::Zero, Bit::One) => { 
                res.push(Bit::Zero);
                carry = Bit::One
            },
            
            (Bit::One, Bit::One, Bit::Zero) => {
                res.push(Bit::Zero);
                carry = Bit::One
            },
            
            (Bit::One, Bit::Zero, Bit::Zero) => {
                res.push(Bit::One);
                carry = Bit::Zero
            },
            
            (Bit::Zero, Bit::One, Bit::Zero) => {
                res.push(Bit::One);
                carry = Bit::Zero
            },
            
            (Bit::Zero, Bit::Zero, Bit::One) => {
                res.push(Bit::One);
                carry = Bit::Zero
            },
            
            (Bit::Zero, Bit::Zero, Bit::Zero) => {
                res.push(Bit::Zero);
                carry = Bit::Zero
            }

        }

    }

    if carry == Bit::One {
        res.push(carry)
    }

    res.reverse();

    res

}