use crate::Integer;

impl Integer {

   pub fn truncate(&mut self) {

      while self.0.len() > 2 && self.0[0] == self.0[1] {
         
         self.0.remove(0);
      
      }

   }

   pub fn truncation(&self) -> Integer {

      let mut result = self.clone();

      while result.0.len() > 2 && result.0[0] == result.0[1] {
         
         result.0.remove(0);
      
      }

      result

   }

}