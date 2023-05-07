use crate::Matrix;

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T> + std::ops::Sub<Output=T> + Clone + std::ops::Add<Output = T> {

    pub fn determinant(&self, neg_one: T) -> Result<T, Box<dyn std::error::Error>> {

        let (rows, columns) = self.dimensions()?;
        
        if rows == columns {

            match rows {

                1 => Ok(self.0[0][0].clone()),

                2 => {
                    
                    let det = (self.0[0][0].clone() * self.0[1][1].clone()) - (self.0[0][1].clone() * self.0[1][0].clone());

                    Ok(det)
                
                },

                _ => {

                    let mut det = self.0[0][0].clone() * self.submatrix(0, 0)?.determinant(neg_one.clone())?;

                    for column in 1..columns {
                        
                        let sub_det = self.submatrix(0, column)?.determinant(neg_one.clone())?;

                        if column % 2 == 0 {

                            det = det + (self.0[0][column].clone() * sub_det)

                        } else {
                            
                            det = det + (self.0[0][column].clone() * neg_one.clone() * sub_det)

                        }

                    }

                    Ok(det)

                }

            }

        } else {
            
            Err("Non Square matrix!")?

        }

    }
    
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_matrix_determinant_0() {

        let a = Matrix(vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,2,9]
        ]);

        assert_eq!(a.determinant(-1).unwrap(), -36);

    }

    #[test]
    fn test_matrix_determinant_for_2_x_2() {

        let a = Matrix(vec![
            vec![6,1],
            vec![5,2]
        ]);

        let b = Matrix(vec![
            vec![5,6],
            vec![2,9]
        ]);

        let c = Matrix(vec![
            vec![4,6],
            vec![7,9]
        ]);

        let d = Matrix(vec![
            vec![4,5],
            vec![7,2]
        ]);

        assert_eq!(a.determinant(-1).unwrap(), 7);

        assert_eq!(b.determinant(-1).unwrap(), 33);

        assert_eq!(c.determinant(-1).unwrap(), -6);

        assert_eq!(d.determinant(-1).unwrap(), -27);
        
    }

    #[test]
    fn test_matrix_determinant_for_3_x_3() {

        let a = Matrix(vec![
            vec![2,3,3],
            vec![2,4,5],
            vec![1,1,2]
        ]);

        let b = Matrix(vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ]);

        let c = Matrix(vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,2,9]
        ]);

        assert_eq!(a.determinant(-1).unwrap(), 3);

        assert_eq!(b.determinant(-1).unwrap(), 0);

        assert_eq!(c.determinant(-1).unwrap(), -36);
        
    }

}
