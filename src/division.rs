
use crate::Int;

pub fn run(a: Int, b: &Int) -> (Int, Int) {

    let mut q = Int {
        bits: vec![]
    };

    let mut r = Int {
        bits: vec![]
    };

    a.bits
        .iter()
        .for_each(|&x| {

            r.bits.push(x);

            match &r.clone().cmp(&b)[..] {
                
                "less" => q.bits.push(0),

                _ => {
                    
                    q.bits.push(1);
                    
                    r = r.clone().sub(&b).unwrap();
                
                }

            };

        });

    while q.bits.len() > 1 && q.bits[0] == 0 {
    
        q.bits.remove(0);
    
    };

    while r.bits.len() > 1 && r.bits[0] == 0 {
    
        r.bits.remove(0);
    
    };

    (q, r)

}