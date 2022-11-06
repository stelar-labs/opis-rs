use crate::Matrix;

impl<T> Matrix<T>

where T: std::ops::Add<Output=T> + Clone {

    pub fn trace(&self) -> Result<T, Box<dyn std::error::Error>> {

        let (rows, columns) = self.dimensions()?;
        
        if rows == columns {

            Ok((1..rows)
                .into_iter()
                .fold(
                    self.0[0][0].clone(),
                    |acc, x|
                    acc + self.0[x][x].clone()
                )
            )

        } else {
            
            Err("Non Square matrix!")?

        }

    }
    
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_matrix_trace() {

        let a = Matrix(vec![vec![-1,2,7,0], vec![3,5,-8,4], vec![1,2,7,-3], vec![4,-2,1,0]]);

        assert_eq!(a.trace().unwrap(), 11)
        
    }

}
