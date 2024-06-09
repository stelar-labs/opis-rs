use crate::Integer;

impl Integer {
   pub fn inversion(&self) -> Integer {
      !self + Integer::one()
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   #[test]
   fn test_one_inversion() {
      assert_eq!(Integer::one().inversion(), Integer::neg_one());
   }
}
