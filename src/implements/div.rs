use crate::Bit;
use crate::Int;
use crate::operators::divisor::main as divisor;

pub fn main(a: Int, b: Int) -> Int {

    let mut res = Int::zero();

    if a.bits == vec![Bit::Zero] {
        res
    }
    
    else if b.bits == vec![Bit::Zero] {
        panic!("a/0 is undefined!")
    }
    
    else {

        let (q, _) = divisor(&a.bits, &b.bits);
        
        res = Int{bits: q, sign: false};

        if res.bits != vec![Bit::Zero] {
            
            match(a.sign, b.sign) {
                
                (false, true) => res.sign = true,
                
                (true, false) => res.sign = true,
                
                _ => ()
            }

        }

        res

    }

}