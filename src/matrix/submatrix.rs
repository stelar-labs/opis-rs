use std::error::Error;

use crate::Matrix;

impl<T> Matrix<T>

where
T: Clone

{

   pub fn submatrix(&self, row: usize, column: usize) -> Result<Matrix<T>, Box<dyn Error>> {

      let (rows, columns) = self.dimensions()?;

      if (row < rows) && (column < columns) {
         
         let mut submatrix = self.clone();

         for r in 0..rows {

            submatrix.0[r].remove(column);

         }

         submatrix.0.remove(row);

         Ok(submatrix)

      } else {

         Err("Row or Column is out of bounds!")?

      }


   }
}