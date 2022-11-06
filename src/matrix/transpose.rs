use crate::Matrix;
use std::error::Error;

impl<T: Clone> Matrix<T> {
    
    pub fn transpose(&self) -> Result<Matrix<T>, Box<dyn Error>> {

        let (_, columns) = self.dimensions()?;
        
        Ok(
            Matrix(
                (0..columns)
                    .into_iter()
                    .map(|column| {
                        self.0
                            .iter()
                            .map(|row| { row[column].clone() })
                            .collect()
                    })
                    .collect()
            )
        )

    }

}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_matrix_transpose() {

        let a = Matrix(vec![vec![2,0], vec![1,1], vec![4,3]]);

        let a_transpose = Matrix(vec![vec![2,1,4], vec![0,1,3]]);

        assert_eq!(a.transpose().unwrap(), a_transpose);
    }

}
