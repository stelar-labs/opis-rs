use crate::Bit;
use crate::Int;
use std::cmp::Ordering;

impl Ord for Int {

    fn cmp(&self, other: &Self) -> Ordering {
        
        if !self.sign && other.sign {
            Ordering::Greater
        }
        
        else if self.sign && !other.sign {
            Ordering::Less
        }
        
        else if !self.sign && !other.sign {
            comparator(&self.magnitude, &other.magnitude)
        }
        
        else {
    
            match comparator(&self.magnitude, &other.magnitude) {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal
            }
    
        }

    }
}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn comparator(a: &Vec<Bit>, b: &Vec<Bit>) -> Ordering {

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
    } else if a_len < b_len {
        Ordering::Less
    } else {

        if a_bits == b_bits {
            Ordering::Equal
        } else {

            while a_bits[0] == b_bits[0] {
                a_bits.remove(0);
                b_bits.remove(0);
            }

            if a_bits[0] == Bit::One {
                Ordering::Greater
            } else {
                Ordering::Less
            }

        }

    }

}