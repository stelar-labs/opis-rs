use crate::Matrix;

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T> + std::ops::Sub<Output=T> + Clone {

    pub fn determinant(&self) -> Result<T, Box<dyn std::error::Error>> {

        let (rows, columns) = self.dimensions()?;
        
        if rows == columns {

            let d1 = (1..rows)
                .into_iter()
                .fold(
                    self.0[0][0].clone(),
                    |acc, x|
                    acc * self.0[x][x].clone()
                );

            let d2 = (0..rows - 1)
                .into_iter()
                .rev()
                .enumerate()
                .fold(
                    self.0[0][rows - 1].clone(),
                    |acc, (i,x)|
                    acc * self.0[i + 1][x].clone()
                );

            Ok(d1 - d2)

        } else {
            
            Err("Non Square matrix!")?

        }

    }
    
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_matrix_determinant() {

        let a = Matrix(vec![vec![6,1], vec![5,2]]);

        assert_eq!(a.determinant().unwrap(), 7)
        
    }

}