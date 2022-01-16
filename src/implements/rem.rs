use crate::Bit;
use crate::Int;
use crate::operators;

pub fn main(a: Int, b: Int) -> Int {

    if a.bits == vec![Bit::Zero] {
        Int::zero()
    }
    
    else if b.bits == vec![Bit::Zero] {
        panic!("a/0 is undefined!")
    }
    
    else {

        let (_, r) = operators::divisor::main(&a.bits, &b.bits);

        let mut res = Int {
            bits: r, sign: false
        };

        if a.sign && res.bits != vec![Bit::Zero] {
            res.sign = true;
        }

        res

    }

}