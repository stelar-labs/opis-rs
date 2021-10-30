
use crate::Int;

pub fn run(a: Int, b: &Int) -> Int {

    if a.bits == vec![0] || b.bits == vec![0] {

        Int {
            bits: vec![0]
        }
    
    } else {

        let mut res: Int = a.clone();

        b.bits
            .iter()
            .skip(1)
            .for_each(|&x| {

                res = res.clone().add(&res);
                
                if x == 1 {
                    
                    res = res.clone().add(&a);
                
                }
            
            });

        res

    }

}