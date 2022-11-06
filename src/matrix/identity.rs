use crate::Matrix;

impl<T: Clone> Matrix<T> {
    
    pub fn identity(size: usize, zero: T, one: T) -> Matrix<T> {
        
        Matrix(
            (0..size)
                .into_iter()
                .map(|row| {
                    (0..size)
                        .into_iter()
                        .map(|column| {
                            if row == column {
                                one.clone()
                            } else {
                                zero.clone()
                            }
                        })
                        .collect()
                })
                .collect()
        )

    }

}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_matrix_identity() {

        let two_identity = Matrix::identity(2, 0, 1);
        
        assert_eq!(two_identity, Matrix(vec![vec![1,0],vec![0,1]]));

    }

}
