use crate::Bit;
use crate::operators::comparator::main as comparator;
use crate::operators::subtractor::main as subtractor;
use std::cmp::Ordering;

pub fn main(a: &Vec<Bit>, b: &Vec<Bit>) -> (Vec<Bit>, Vec<Bit>) {

    let mut q: Vec<Bit> = vec![Bit::Zero];

    let mut r: Vec<Bit> = vec![Bit::Zero];

    a.iter()
        .for_each(|x| {

            r.push(x.clone());

            while r.len() > 1 && r[0] == Bit::Zero {
                r.remove(0);
            };

            if comparator(&r, b) == Ordering::Greater {

                q.push(Bit::One);
                
                r = subtractor(&r, b);
                
            }
            
            else if comparator(&r, b) == Ordering::Equal {

                q.push(Bit::One);

                r = vec![Bit::Zero]

            }
            
            else {
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