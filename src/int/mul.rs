use crate::Bit;
use crate::Int;
use std::ops::Mul;
use crate::int::add::adder;

impl Mul for Int {

    type Output = Self;
    
    fn mul(self, other: Self) -> Self {

        let product: Vec<Bit> = multiplier(self.magnitude, other.magnitude);
        
        match (self.sign, other.sign) {

            (false, false) => Int { magnitude: product, sign: false },

            (true, true) => Int { magnitude: product, sign: false },

            (false, true) => Int { magnitude: product, sign: true },

            (true, false) => Int { magnitude: product, sign: true }

        } 

    }

}

impl Mul for &Int {

    type Output = Int;
    
    fn mul(self, b: Self) -> Int {
        self.clone() * b.clone()
    }

}

fn multiplier(a: Vec<Bit>, b: Vec<Bit>) -> Vec<Bit> {

    let mut res: Vec<Bit> = a.clone();

    b.iter()
        .skip(1)
        .for_each(|x| {

            res = adder(res.clone(), res.clone());
            
            if x == &Bit::One {
                res = adder(res.clone(), a.clone());
            }
        
        }
    );

    res
    
}
