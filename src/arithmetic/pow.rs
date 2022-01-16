use crate::Bit;
use crate::Int;

pub fn main(a: &Int, e:&Int) -> Int {

    if a == &Int::zero() {
        Int::zero()
    }
    
    else if e == &Int::zero() {
        Int::one()
    }
    
    else {

        let mut res: Int = a.clone();

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