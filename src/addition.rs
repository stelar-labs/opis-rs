
use crate::Int;

pub fn run(mut a: Int, mut b: Int) -> Int {

    let mut res = Int {
        bits: vec![]
    };

    let mut carry = 0;

    while a.bits.len() > 0 || b.bits.len() > 0 {

        let a_bit = match a.bits.pop() {
            Some(r) => r,
            None => 0
        };

        let b_bit = match b.bits.pop() {
            Some(r) => r,
            None => 0
        };

        let addition = carry + a_bit + b_bit;

        match addition {
            3 => { res.bits.push(1); carry = 1 },
            2 => { res.bits.push(0); carry = 1 },
            1 => { res.bits.push(1); carry = 0 },
            _ => { res.bits.push(0); carry = 0 }
        }

    }

    if carry != 0 {
        res.bits.push(1)
    }

    res.bits.reverse();

    res

}
