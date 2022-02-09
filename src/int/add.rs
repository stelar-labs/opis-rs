use std::ops::{Add, AddAssign};
use std::cmp::Ordering;

use crate::Bit;
use crate::Int;

use crate::int::cmp::comparator;
use crate::int::sub::subtractor;


impl Add for Int {
    
    type Output = Self;
    
    fn add(self, b: Self) -> Self {

        match (self.bits[0], b.bits[0]) {

            (Bit::Zero, Bit::Zero) => Int {
                bits: [
                    vec![Bit::Zero],
                    adder(self.bits[1..].to_vec(), b.bits[1..].to_vec())
                ].concat()
            },

            (Bit::One, Bit::One) => Int {
                bits: [
                    vec![Bit::One],
                    adder(self.bits[1..].to_vec(), b.bits[1..].to_vec())
                ].concat()
            },

            (Bit::One, Bit::Zero) => {
                
                match comparator(&self.bits[1..].to_vec(), &b.bits[1..].to_vec()) {
                    Ordering::Equal => Int::zero(),
                    Ordering::Greater => Int {
                        bits: [
                            vec![Bit::One],
                            subtractor(self.bits[1..].to_vec(), b.bits[1..].to_vec())
                        ].concat()
                    },
                    Ordering::Less => Int {
                        bits: [
                            vec![Bit::Zero],
                            subtractor(b.bits[1..].to_vec(), self.bits[1..].to_vec())
                        ].concat()
                    }

                }
            },

            (Bit::Zero, Bit::One) => {
                
                match comparator(&self.bits[1..].to_vec(), &b.bits[1..].to_vec()) {
                    Ordering::Equal => Int::zero(),
                    Ordering::Greater => Int {
                        bits: [
                            vec![Bit::Zero],
                            subtractor(self.bits[1..].to_vec(), b.bits[1..].to_vec())
                        ].concat()
                    },
                    Ordering::Less => Int {
                        bits: [
                            vec![Bit::One],
                            subtractor(b.bits[1..].to_vec(), self.bits[1..].to_vec())
                        ].concat()
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