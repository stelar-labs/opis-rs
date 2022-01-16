use crate::Bit;
use crate::Int;

pub fn main(mut a: Int) -> Int {
        
    a.bits = a.bits
        .iter()
        .map(|x| {
            match x {
                Bit::One => Bit::Zero,
                Bit::Zero => Bit::One
            }
        })
        .collect();

    a

}