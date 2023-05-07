use crate::{Integer, Bit};
use std::cmp::Ordering;

impl PartialEq for Integer {

    fn eq(&self, b: &Self) -> bool {

        if self.0.len() == b.0.len() {

            self.0 == b.0

        } else {

            let mut trim_a = self.0.clone();

            while trim_a.len() > 2 && trim_a[0] == trim_a[1] {

                trim_a.remove(0);

            }

            let mut trim_b = b.0.clone();

            while trim_b.len() > 2 && trim_b[0] == trim_b[1] {

                trim_b.remove(0);

            }
            
            trim_a == trim_b

        }

    }

}

impl Eq for Integer {}

impl Ord for Integer {

    fn cmp(&self, b: &Self) -> Ordering {

        let mut result = Ordering::Equal;

        if self.0[0] == Bit::Zero && b.0[0] == Bit::One {

            result = Ordering::Greater

        } else if self.0[0] == Bit::One && b.0[0] == Bit::Zero {

            result = Ordering::Less

        } else {
            
            let precision = if self.0.len() > b.0.len() {
                self.0.len()
            } else {
                b.0.len()
            };

            let a_diff = precision - self.0.len();

            let b_diff = precision - b.0.len();
            
            for i in 1..precision {

                let a_bit = if i >= a_diff {
                    self.0[i - a_diff]
                } else {
                    self.0[0]
                };
        
                let b_bit = if i >= b_diff {
                    b.0[i - b_diff]
                } else {
                    b.0[0]
                };

                match (a_bit, b_bit) {
                    (Bit::Zero, Bit::One) => { result = Ordering::Less; break; },
                    (Bit::One, Bit::Zero) => { result = Ordering::Greater; break; },
                    _ => ()
                }
            }
        }
        result
    }
}

impl PartialOrd for Integer {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_cmp() {
        assert!(Integer::three() > Integer::two());
    }

}
