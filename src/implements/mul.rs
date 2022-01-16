use crate::Bit;
use crate::Int;
use crate::operators;

pub fn main(a: Int, b: Int) -> Int {

    let mut res = Int::zero();

    if a.bits != vec![Bit::Zero] && b.bits != vec![Bit::Zero] {

        res = a.clone();

        b.bits
            .iter()
            .skip(1)
            .for_each(|x| {

                res.bits = operators::adder::main(&res.bits, &res.bits);
                
                if x == &Bit::One {
                    res.bits = operators::adder::main(&res.bits, &a.bits);
                }
            
            });

        match(a.sign, b.sign) {
            (false, true) => res.sign = true,
            (true, false) => res.sign = true,
            _ => ()
        }

    }

    res

}