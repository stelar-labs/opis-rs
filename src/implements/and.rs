use crate::Bit;
use crate::Int;

pub fn main(mut a: Int, mut b: Int) -> Int {

    let mut res: Vec<Bit> = Vec::new();

    while !a.bits.is_empty() || !b.bits.is_empty() {

        let a_bit: Bit =
            match a.bits.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        let b_bit: Bit =
            match b.bits.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        let and_bit: Bit =
            match (a_bit, b_bit) {
                (Bit::One, Bit::One) => Bit::One,
                _ => Bit::Zero
            };

        res.push(and_bit)

    }

    res.reverse();

    Int {
        bits: res,
        sign: false
    }

}