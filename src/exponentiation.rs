
use crate::Int;

pub fn run(a: Int, b: &Int) -> Int {

    if a.bits == vec![0] {

        Int::zero()

    } else if b.bits == vec![0] {

        Int::one()

    } else {

        let mut res: Int = a.to_owned();

        b.bits
            .iter()
            .skip(1)
            .for_each(|&x| {

                res = res.to_owned().mul(&res);
                
                if x == 1 {

                    res = res.to_owned().mul(&a)
                
                }
            
            });

        res

    }

}