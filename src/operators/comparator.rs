use crate::Bit;
use std::cmp::Ordering;

pub fn main(a: &Vec<Bit>, b: &Vec<Bit>) -> Ordering {

    let mut a_bits = a.clone();
    
    let mut b_bits = b.clone();

    while a_bits.len() > 1 && a_bits[0] == Bit::Zero {
        a_bits.remove(0);
    };

    while b_bits.len() > 1 && b_bits[0] == Bit::Zero {
        b_bits.remove(0);
    };

    let a_len = a_bits.len();
    
    let b_len = b_bits.len();

    if a_len > b_len {
        Ordering::Greater
    }
    
    else if a_len < b_len {
        Ordering::Less
    }
    
    else {

        if a_bits == b_bits {
            Ordering::Equal
        }
        
        else {

            while a_bits[0] == b_bits[0] { 
                
                a_bits.remove(0);
                
                b_bits.remove(0);

            }

            if a_bits[0] == Bit::One {
                Ordering::Greater
            }
            
            else {
                Ordering::Less
            }

        }

    }

}