use crate::Bit;
use crate::Int;
use std::cmp::Ordering;

impl Ord for Int {

    fn cmp(&self, b: &Self) -> Ordering {

        let mut result = Ordering::Equal;

        if self.bits[0] == Bit::Zero && b.bits[0] == Bit::One {

            result = Ordering::Greater

        } else if self.bits[0] == Bit::One && b.bits[0] == Bit::Zero {

            result = Ordering::Less

        } else {
            
            let precision =
                if self.bits.len() > b.bits.len() {
                    self.bits.len()
                } else {
                    b.bits.len()
                };
            
            for i in 1..precision {

                let a_bit = if self.bits.len() == precision {

                    self.bits[i]
                
                } else {

                    let diff = precision - self.bits.len();

                    if i > diff {
                        self.bits[i - diff]
                    } else {
                        self.bits[0]
                    }
                    
                };

                let b_bit = if b.bits.len() == precision {

                    b.bits[i]

                } else {

                    let diff = precision - b.bits.len();

                    if i > diff {
                        b.bits[i - diff]
                    } else {
                        b.bits[0]
                    }
                
                };

                match (a_bit, b_bit) {

                    (Bit::Zero, Bit::One) => {
                        result = Ordering::Less;
                        break;
                    },

                    (Bit::One, Bit::Zero) => {
                        result = Ordering::Greater;
                        break;
                    },

                    _ => ()

                }
                
            }
        
        }

        result

    }
}

impl PartialOrd for Int {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        Some(self.cmp(other))
    
    }

}
