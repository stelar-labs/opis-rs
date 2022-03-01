
use std::ops::Div;
use std::cmp::Ordering;

use crate::Bit;
use crate::Int;

use crate::int::cmp::comparator;
use crate::int::sub::subtractor;

impl Div for Int {

    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        
        if self == Int::zero() {
            Int::zero()
        }
        
        else if other == Int::zero() {
            panic!("a/0 is undefined!")
        }
        
        else {

            let (q, _) = divisor(self.magnitude, other.magnitude);

            match (self.sign, other.sign) {

                (false, false) => Int { magnitude: q, sign: false },
    
                (true, true) => Int { magnitude: q, sign: false },
    
                (false, true) => Int { magnitude: q, sign: true },
    
                (true, false) => Int { magnitude: q, sign: true }
    
            }

        }

    }

}

impl Div for &Int {

    type Output = Int;
    
    fn div(self, b: Self) -> Int {
        self.clone() / b.clone()
    }

}

pub fn divisor(n: Vec<Bit>, d: Vec<Bit>) -> (Vec<Bit>, Vec<Bit>) {

    let mut q: Vec<Bit> = vec![Bit::Zero];

    let mut r: Vec<Bit> = vec![Bit::Zero];

    n.iter()
        .for_each(|x| {

            r.push(x.clone());

            while r.len() > 1 && r[0] == Bit::Zero {
                r.remove(0);
            };

            if comparator(&r, &d) == Ordering::Greater {

                q.push(Bit::One);
                
                r = subtractor(r.clone(), d.clone());
                
            } else if comparator(&r, &d) == Ordering::Equal {

                q.push(Bit::One);

                r = vec![Bit::Zero]

            } else {
                q.push(Bit::Zero)
            };

        });

    while q.len() > 1 && q[0] == Bit::Zero {
        q.remove(0);
    };

    while r.len() > 1 && r[0] == Bit::Zero {
        r.remove(0);
    };

    (q, r)

}