use std::error::Error;
use crate::Matrix;

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T> + std::ops::Sub<Output=T> + Clone {

   pub fn minors(&self) -> Result<Matrix<T>, Box<dyn Error>> {

      let (rows, columns) = self.dimensions()?;

      if rows == columns {

         let mut minors_matrix = Matrix(vec![vec![]; rows]);

         for i in 0..rows {

            for j in 0..columns {

               let mut minor_matrix = self.clone();

               for x in 0..rows {

                  minor_matrix.0[x].remove(j);

               }

               minor_matrix.0.remove(i);

               let minor = minor_matrix.determinant()?;

               minors_matrix.0[i].push(minor)

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
      
      assert_eq!(a.minors().unwrap(), m)
   
   }

}
