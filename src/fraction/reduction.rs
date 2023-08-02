use crate::{Fraction, Integer};

impl Fraction {

    pub fn reduce(&mut self) {

        if self.0 != Integer::zero() && self.1 != Integer::zero() {
            
            let (gcd, _, _) = self.0.ext_gcd(&self.1);

            if gcd != Integer::one() {
        
                self.0 = (&self.0 / &gcd).unwrap();
            
                self.1 = (&self.1 / &gcd).unwrap();

            }

        }

    }

    pub fn reduction(&self) -> Fraction {

        if self.0 != Integer::zero() && self.1 != Integer::zero() {

            let (gcd, _, _) = self.0.ext_gcd(&self.1);

            let n = (&self.0 / &gcd).unwrap();

            let d = (&self.1 / &gcd).unwrap();

            Fraction(n, d)

        } else {

            self.clone()

        }

    }

}
