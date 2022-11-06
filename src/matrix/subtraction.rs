
use crate::Matrix;
use std::error::Error;
use std::ops::Sub;

impl<T> Sub for Matrix<T>

where T: std::ops::Add + std::clone::Clone + Sub<Output = T> {
    
    type Output = Result<Matrix<T>, Box<dyn Error>>;

    fn sub(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>> {

        &self - &b

    }
    
}

impl<T> Sub for &Matrix<T>

where T: std::ops::Add + std::clone::Clone + Sub<Output = T> {

    type Output = Result<Matrix<T>, Box<dyn Error>>;

    fn sub(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>> {

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
                                .map(|column| self.0[row][column].clone() - b.0[row][column].clone())
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
