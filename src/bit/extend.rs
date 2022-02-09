
use crate::Bit;

pub fn right(mut bits: Vec<Bit>, length: usize) -> Vec<Bit> {
    
    while bits.len() < length {
        bits = [vec![bits[0]], bits].concat()
    }
    
    bits

}

pub fn left(mut bits: Vec<Bit>, mut amount: usize) -> Vec<Bit> {

    while amount != 0 {
        
        bits.push(Bit::Zero);
        
        amount -= 1
        
    }

    bits

}