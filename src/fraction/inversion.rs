use crate::{Fraction, Bit};

impl Fraction {
   
   pub fn inversion(&self) -> Fraction {

      // if negative 
      if self.sign() {

         // make n positive 
         let n = if self.0.0[0] == Bit::One {
            self.0.inversion()
         } else {
            self.0.clone()
         };

         // make d positive 
         let d = if self.0.0[0] == Bit::One {
            self.0.inversion()
         } else {
            self.0.clone()
         };
         
         Fraction(n, d)

      // else is positive
      } else {

         // make n negative 
         let n = if self.0.0[0] == Bit::Zero {
            self.0.inversion()
         } else {
            self.0.clone()
         };

         // make d positive
         let d = if self.0.0[0] == Bit::One {
            self.0.inversion()
         } else {
            self.0.clone()
         };

         Fraction(n, d)

      }
   
   }

}
