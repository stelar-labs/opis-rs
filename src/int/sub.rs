use std::cmp::Ordering;
use std::ops::{Sub, SubAssign};

use crate::Bit;
use crate::Int;

use crate::int::add::adder;
use crate::int::cmp::comparator;

impl Sub for Int {
    
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        
        match (self.sign, other.sign) {

            (false, false) => {
    
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
    
            },
    
            (true, true) => {
    
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
    
            (true, false) => Int {
                magnitude: adder(self.magnitude, other.magnitude),
                sign: true
            },
    
            (false, true) => Int {
                magnitude: adder(self.magnitude, other.magnitude),
                sign: false
            }
    
        }

    }
}

impl Sub for &Int {

    type Output = Int;
    
    fn sub(self, b: Self) -> Int {
        self.clone() - b.clone()
    }

}

impl SubAssign for Int {
    
    fn sub_assign(&mut self, b: Self) {
        *self = self.clone() - b
    }

}

pub fn subtractor(mut a: Vec<Bit>, mut b: Vec<Bit>) -> Vec<Bit> {
    
    let mut res: Vec<Bit> = Vec::new();

    while !a.is_empty() || !b.is_empty() {

        let a_bit: Bit = match a.pop() { Some(r) => r, None => Bit::Zero };

        let b_bit: Bit = match b.pop() { Some(r) => r, None => Bit::Zero };
        
        match (a_bit, b_bit) {
            
            (Bit::Zero, Bit::Zero) => res.push(Bit::Zero),
            
            (Bit::One, Bit::One) => res.push(Bit::Zero),
            
            (Bit::One, Bit::Zero) => res.push(Bit::One),
            
            (Bit::Zero, Bit::One) => {
                
                let mut i: usize = a.len() - 1;
                
                let mut borrowed: bool = false;
                
                while !borrowed {
                    
                    if a[i] == Bit::One {    
                        a[i] = Bit::Zero;
                        borrowed = true;
                    }
                    
                    else {
                        a[i] = Bit::One;
                        i -= 1;
                    }

                }
                
                res.push(Bit::One);

            }
            
        }

    }

    res.reverse();

    while res.len() > 1 && res[0] == Bit::Zero {
        res.remove(0);
    }

    res

}