use crate::Bit;
use crate::Int;

pub fn exponentiation(a: &Int, e:&Int) -> Int {

    if a == &Int::zero() {
        Int::zero()
    }
    
    else if e == &Int::zero() {
        Int::one()
    }
    
    else if e.bits[0] == Bit::One {
        panic!("Non Integer result for negative exponent!")
    }

    else {

        let mut res: Int = Int {bits: a.bits[1..].to_vec()};

        e.bits
        .iter()
        .skip(1)
        .for_each(|x| {

            res = &res * &res;

            if x == &Bit::One {
                res = &res * a
            }
            
        });

        res
        
    }

}