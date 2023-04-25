use crate::Matrix;
use std::error::Error;

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T> + Clone {

   pub fn cofactors(&self, neg_one: T, one: T) -> Result<Matrix<T>, Box<dyn Error>> {

      let (rows, columns) = self.dimensions()?;

      if rows == columns {

         let mut cofactors_matrix = Matrix(vec![vec![]; rows]);

         for i in 0..rows {

            for j in 0..columns {

               let i_plus_j = i + j;

               let rem = i_plus_j % 2;

               if rem == 0 {

                  let cofactor = one.clone() * self.0[i][j].clone();

                  cofactors_matrix.0[i].push(cofactor);

               } else {

                  let cofactor = neg_one.clone() * self.0[i][j].clone();

                  cofactors_matrix.0[i].push(cofactor);
                   
               }

            }

         }

         Ok(cofactors_matrix)

      } else {
         
         Err("Non Square matrix!")?
      }

   }

}

#[cfg(test)]
mod tests {
   
   use super::*;
   
   #[test]
   
   fn test_matrix_cofactors() {
      
      let a = Matrix(vec![vec![3,-1,-2], vec![3,1,-1], vec![3,4,2]]);

      let c = Matrix(vec![vec![3,1,-2], vec![-3,1,1], vec![3,-4,2]]);
      
      assert_eq!(a.cofactors(-1,1).unwrap(), c)
   
   }

}
