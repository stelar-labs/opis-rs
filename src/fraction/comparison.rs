use crate::Fraction;
use std::cmp::Ordering;

impl Ord for Fraction {

    fn cmp(&self, b: &Self) -> Ordering {

        if self.sign() == false && b.sign() == true {

            Ordering::Less

        } else if self.sign() == true && b.sign() == false {  

            Ordering::Greater

        } else {

            let ad = &self.0 * &b.1;

            let bc = &self.1 * &b.0;

            ad.cmp(&bc)

        }

    }
    
}

impl PartialOrd for Fraction {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        
        Some(self.cmp(other))
    
    }

}
