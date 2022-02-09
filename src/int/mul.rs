use crate::Bit;
use crate::Int;
use std::ops::Mul;
use crate::int::add::adder;

impl Mul for Int {

    type Output = Self;
    
    fn mul(self, b: Self) -> Self {

        let product: Vec<Bit> = multiplier(self.bits[1..].to_vec(), b.bits[1..].to_vec());
        
        match (self.bits[0], b.bits[0]) {

            (Bit::Zero, Bit::Zero) => Int {bits: [vec![Bit::Zero], product].concat()},

            (Bit::One, Bit::One) => Int {bits: [vec![Bit::Zero], product].concat()},

            (Bit::Zero, Bit::One) => Int {bits: [vec![Bit::One], product].concat()},

            (Bit::One, Bit::Zero) => Int {bits: [vec![Bit::One], product].concat()}

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

    b
    .iter()
    .skip(1)
    .for_each(|x| {

        res = adder(res.clone(), res.clone());
        
        if x == &Bit::One {
            res = adder(res.clone(), a.clone());
        }
    
    });

    res
    
}
