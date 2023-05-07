use crate::Matrix;
use std::error::Error;

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T>
+ std::ops::Sub<Output=T>
+ Clone
+ std::cmp::PartialOrd
+ std::ops::Div<Output=Result<T, Box<dyn Error>>>
+ std::ops::Add<Output=T> + std::fmt::Debug {
    
    pub fn inverse_div_err(&self, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>> {

        let (rows, columns) = self.dimensions()?;

        if rows == columns {

            let determinant = self.determinant(neg_one.clone())?;

            if determinant != zero {

                let minors_matrix = self.minors(neg_one.clone())?;

                let cofactors_matrix = minors_matrix.cofactors(neg_one)?;

                let transpose_matrix = cofactors_matrix.transpose()?;

                let inv_det = (one / determinant)?;

                let inverse_matrix = transpose_matrix * inv_det;
                
                Ok(inverse_matrix)

            } else {

                Err("Matrix is not invertible!")?

            }

        } else {
            
            Err("Non Square matrix!")?

        }

    }

}

impl<T> Matrix<T>
where T: std::ops::Mul<Output=T>
+ std::ops::Sub<Output=T>
+ Clone
+ std::cmp::PartialOrd
+ std::ops::Div<Output=T>
+ std::ops::Add<Output=T> + std::fmt::Debug {
    
    pub fn inverse(&self, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>> {

        let (rows, columns) = self.dimensions()?;

        if rows == columns {

            let determinant = self.determinant(neg_one.clone())?;

            if determinant != zero {

                let minors_matrix = self.minors(neg_one.clone())?;

                let cofactors_matrix = minors_matrix.cofactors(neg_one)?;

                let transpose_matrix = cofactors_matrix.transpose()?;

                let inv_det = one / determinant;

                let inverse_matrix = transpose_matrix * inv_det;
                
                Ok(inverse_matrix)

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
    
    use crate::Fraction;
    use super::*;

    #[test]
    fn test_matrix_inverse_0() {

        let a = Matrix(vec![
            vec![Fraction::from(&1_u8), (&2_u8).into(), (&3_u8).into()],
            vec![(&4_u8).into(), (&5_u8).into(), (&6_u8).into()],
            vec![(&7_u8).into(), (&2_u8).into(), (&9_u8).into()],
        ]);

        let i = Matrix(vec![
            vec![("-11/12").try_into().unwrap(), ("1/3").try_into().unwrap(), ("1/12").try_into().unwrap()],
            vec![("-1/6").try_into().unwrap(), ("1/3").try_into().unwrap(), ("-1/6").try_into().unwrap()],
            vec![("3/4").try_into().unwrap(), ("-1/3").try_into().unwrap(), ("1/12").try_into().unwrap()],
        ]);

        assert_eq!(
            a.inverse_div_err(
                ("-1/1").try_into().unwrap(),
                (&0_u8).into(),
                (&1_u8).into()
            ).unwrap(),
            i
        );
         
    }

    // #[test]
    // fn test_matrix_inverse_for_i8() {

    //     let a = Matrix(vec![vec![1,2], vec![1,3]]);

    //     let i = Matrix(vec![vec![3,-2], vec![-1,1]]);

    //     assert_eq!(a.inverse(-1,0,1).unwrap(), i);
         
    // }

    // #[test]
    // fn test_matrix_inverse_for_fraction() {

    //     let a = Matrix(vec![
    //         vec![Fraction::from(&2_u8), (&3_u8).into(), (&3_u8).into()],
    //         vec![(&2_u8).into(), (&4_u8).into(), (&5_u8).into()],
    //         vec![(&1_u8).into(), (&1_u8).into(), (&2_u8).into()],
    //     ]);

    //     let i = Matrix(vec![
    //         vec![(&1_u8).into(), ("-1/1").try_into().unwrap(), (&1_u8).into()],
    //         vec![("1/3").try_into().unwrap(), ("1/3").try_into().unwrap(), ("-4/3").try_into().unwrap()],
    //         vec![("-2/3").try_into().unwrap(), ("1/3").try_into().unwrap(), ("2/3").try_into().unwrap()],
    //     ]);

    //     assert_eq!(
    //         a.inverse_div_err(
    //             ("-1/1").try_into().unwrap(),
    //             (&0_u8).into(),
    //             (&1_u8).into()
    //         ).unwrap(),
    //         i
    //     );
         
    // }

}
