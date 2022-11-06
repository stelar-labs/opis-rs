use crate::Matrix;
use std::error::Error;

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T>
+ std::ops::Sub<Output=T>
+ Clone
+ std::cmp::PartialOrd
+ std::ops::Div<Output=T>
+ std::ops::Add<Output=T> {
    
    pub fn inverse(&self, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>> {

        let (rows, columns) = self.dimensions()?;

        if rows == columns {

            let determinant = self.determinant()?;

            if determinant != zero {

                let mut a = self.clone();

                let identity = Matrix::identity(rows, zero.clone(), one.clone());

                let mut inverse = identity.clone();

                for c in 0..columns {

                    if self.0[c][c] != one {
                    
                        if self.0[c][c] != zero {

                            inverse.0[c] = inverse.0[c]
                                .iter()
                                .map(|x| x.clone() / a.0[c][c].clone())
                                .collect();

                            a.0[c] = a.0[c]
                                .iter()
                                .map(|x| x.clone() / a.0[c][c].clone())
                                .collect();

                        } else {

                            inverse.0[c] = inverse.0[c]
                                .iter()
                                .map(|x| x.clone() + one.clone())
                                .collect();

                            a.0[c] = a.0[c]
                                .iter()
                                .map(|x| x.clone() + one.clone())
                                .collect();

                        }
                    
                    }

                    for r in 0..rows {

                        if c != r {

                            inverse.0[r] = inverse.0[r]
                                .iter()
                                .enumerate()
                                .map(|(i,x)| x.clone() - (a.0[r][c].clone() * inverse.0[c][i].clone()))
                                .collect();
                            
                            a.0[r] = a.0[r]
                                .iter()
                                .enumerate()
                                .map(|(i,x)| x.clone() - (a.0[r][c].clone() * a.0[c][i].clone()))
                                .collect();

                        }

                    }

                }

                if (self.clone() * inverse.clone())? == identity {
                    
                    Ok(inverse)

                } else {
                    
                    Err("Cannot find inverse!")?

                }
                

            } else {

                Err("Matrix is not invertible!")?

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
    fn test_matrix_inverse() {

        let a = Matrix(vec![vec![1,2], vec![1,3]]);

        let a_inverse = Matrix(vec![vec![3,-2], vec![-1,1]]);

        assert_eq!(a.inverse(0,1).unwrap(), a_inverse);
         
    }

}
