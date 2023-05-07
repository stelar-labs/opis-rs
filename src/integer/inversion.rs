use crate::Integer;

impl Integer {
   
   pub fn inversion(&self) -> Integer {

      !self + Integer::one()

   }

}

#[cfg(test)]
mod tests {
   
   use crate::Bit;

   use super::*;
   
   #[test]
   fn test_neg_five_inversion() {

      let neg_five = Integer(vec![Bit::One,Bit::Zero,Bit::One,Bit::One]);

      let pos_five = Integer(vec![Bit::Zero,Bit::One,Bit::Zero,Bit::One]);
      
      assert_eq!(neg_five.inversion(), pos_five);
   
   }

}
