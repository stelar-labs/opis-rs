use crate::Int;
use crate::operators;
use std::cmp::Ordering;

pub fn main(a: &Int, b: &Int) -> Ordering {
        
    if !a.sign && b.sign {
        Ordering::Greater
    }
    
    else if a.sign && !b.sign {
        Ordering::Less
    }
    
    else if !a.sign && !b.sign {
        operators::comparator::main(&a.bits, &b.bits)
    }
    
    else {

        match operators::comparator::main(&a.bits, &b.bits) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal
        }

    }

}