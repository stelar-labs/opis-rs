
use std::{ops::{Mul, Add}, error::Error};

use crate::Matrix;

impl<T> Mul for Matrix<T>
where T: Mul + Mul<Output = T> + Add<Output=T> + Clone{

    type Output = Result<Matrix<T>, Box<dyn Error>>;

    fn mul(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>> {
        &self * &b
    }

}

impl<T> Mul<T> for Matrix<T>
where T: Mul + Mul<Output = T> + Clone {
    
    type Output = Matrix<T>;

    fn mul(self, b: T) -> Matrix<T> {
        &self * &b
    }

}

impl<T> Mul<&T> for &Matrix<T>
where T: Mul + Mul<Output = T> + Clone {
    
    type Output = Matrix<T>;

    fn mul(self, b: &T) -> Matrix<T> {
        Matrix(
            self.0
                .iter()
                .map(|row| {
                    row
                        .iter()
                        .map(|x| x.clone() * b.clone())
                        .collect()
                })
                .collect()
        )
    }

}

impl<T> Mul for &Matrix<T>
where T: Mul + Mul<Output = T> + Add<Output=T> + Clone{

    type Output = Result<Matrix<T>, Box<dyn Error>>;

    fn mul(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>> {

        let (a_rows, a_columns) = self.dimensions()?;

        let (b_rows, b_columns) = b.dimensions()?;

        if a_columns == b_rows {

            if a_columns == 0 {

                Ok(Matrix(vec![]))

            } else {

                Ok(
                    Matrix(
                        (0..a_rows)
                            .into_iter()
                            .map(|a_row_i| {
                                (0..b_columns)
                                    .into_iter()
                                    .map(|b_column_i| {

                                        let b_row: Vec<T> = b.0.iter().map(|x| x[b_column_i].clone()).collect();

                                        (1..a_columns)
                                            .into_iter()
                                            .fold(
                                                self.0[a_row_i][0].clone() * b_row[0].clone(),
                                                |acc, x| acc + (self.0[a_row_i][x].clone() * b_row[x].clone()))

                                    })
                                    .collect()
                            })
                            .collect()
                    )
                )

            }

        } else {

            Err("Product of the Matrices is undefined!")?

        }
        
    }
    
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_matrix_scalar_multiplication() {

        let a = Matrix(vec![vec![2_i8,1], vec![4,6], vec![-1,0]]);

        let a_3 = Matrix(vec![vec![6_i8,3], vec![12,18], vec![-3,0]]);

        assert_eq!(a * 3_i8, a_3)
        
    }

    #[test]
    fn test_matrix_multiplication() {

        let a = Matrix(vec![vec![1,2,4], vec![2,6,0]]);

        let b = Matrix(vec![vec![4,1,4,3], vec![0,-1,3,1], vec![2,7,5,2]]);

        let ab = Matrix(vec![vec![12,27,30,13], vec![8,-4,26,12]]);

        assert_eq!((a * b).unwrap(), ab)
        
    }

}
