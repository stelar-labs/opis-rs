
use crate::Int;

pub fn run(a: Int, b: &Int) -> Int {

    let zero = Int {
        bits: vec![0]
    };

    if a.bits == zero.bits {

        zero

    } else if b.bits == zero.bits {

        Int {
            bits: vec![1]
        }

    } else {

        let mut res: Int = a.clone();

        b.bits
            .iter()
            .skip(1)
            .for_each(|&x| {

                res = res.clone().mul(&res);
                
                if x == 1 {
                    res = res.clone().mul(&a)
                }
            
            });

        res

    }

}