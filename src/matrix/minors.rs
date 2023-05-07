use std::error::Error;
use crate::Matrix;

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T> + std::ops::Sub<Output=T> + Clone + std::ops::Add<Output = T> + std::fmt::Debug {

   pub fn minors(&self, neg_one: T) -> Result<Matrix<T>, Box<dyn Error>> {

      let (rows, columns) = self.dimensions()?;

      if rows == columns {

         let mut minors_matrix = Matrix(vec![vec![]; rows]);

         for row in 0..rows {

            for column in 0..columns {

               let minor_matrix = self.submatrix(row, column)?;

               let minor = minor_matrix.determinant(neg_one.clone())?;

               minors_matrix.0[row].push(minor)

            }

         }

         Ok(minors_matrix)

      } else {
         
         Err("Non Square matrix!")?
         
      }

   }
    
}

#[cfg(test)]
mod tests {
   
   use super::*;
   
   #[test]
   fn test_matrix_minors() {
      
      let a = Matrix(vec![vec![2,3,3], vec![2,4,5], vec![1,1,2]]);

      let m = Matrix(vec![vec![3,-1,-2], vec![3,1,-1], vec![3,4,2]]);
      
      assert_eq!(a.minors(-1).unwrap(), m)
   
   }

   #[test]
   fn test_matrix_minors_1() {
      
      let a = Matrix(vec![vec![1,2,3], vec![4,5,6], vec![7,2,9]]);

      let m = Matrix(vec![vec![33,-6,-27], vec![12,-12,-12], vec![-3,-6,-3]]);
      
      assert_eq!(a.minors(-1).unwrap(), m)
   
   }

}
