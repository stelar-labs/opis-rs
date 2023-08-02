use crate::Fraction;

impl PartialEq for Fraction {

    fn eq(&self, b: &Self) -> bool {
        
        if self.sign() == b.sign() {

            let self_red = self.reduction();

            let b_red = b.reduction();

            self_red.0 == b_red.0 && self_red.1 == b_red.1

        } else {

            false

        }
    
    }

}

impl Eq for Fraction {}
