
use crate::Matrix;
use std::error::Error;
use std::ops::Add;

impl<T> Add for Matrix<T>

where T: std::ops::Add + std::clone::Clone + Add<Output = T> {
    
    type Output = Result<Matrix<T>, Box<dyn Error>>;

    fn add(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>> {

        &self + &b

    }
    
}

impl<T> Add for &Matrix<T>

where T: std::ops::Add + std::clone::Clone + Add<Output = T> {

    type Output = Result<Matrix<T>, Box<dyn Error>>;

    fn add(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>> {

        let a_dimensions = self.dimensions()?;

        let b_dimensions = b.dimensions()?;

        if a_dimensions == b_dimensions {

            Ok(
                Matrix(
                    (0..a_dimensions.0)
                        .into_iter()
                        .map(|row| {
                            (0..a_dimensions.1)
                                .into_iter()
                                .map(|column| self.0[row][column].clone() + b.0[row][column].clone())
                                .collect()
                        })
                        .collect()
                )
            )
            
        } else {
            
            Err("Matrices have different dimensions!")?
        
        }

    }

}


#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_matrix_addition() {

        let a = Matrix(vec![vec![1,0,3], vec![1,-1,4], vec![2,-1,0]]);

        let b = Matrix(vec![vec![4,2,-1], vec![1,0,6], vec![3,1,4]]);

        let a_plus_b = Matrix(vec![vec![5,2,2], vec![2,-1,10], vec![5,0,4]]);

        assert_eq!((a + b).unwrap(), a_plus_b)
        
    }

}
