use std::error::Error;
use crate::Matrix;

impl<T> Matrix<T> {

    pub fn dimensions(&self) -> Result<(usize, usize), Box<dyn Error>> {
        
        if self.0.is_empty() || self.0[0].len() == 0 {

            Ok((0,0))

        } else {

            match self.0.iter().all(|x| x.len() == self.0[0].len()) {

                true => Ok((self.0.len(), self.0[0].len())),

                false => Err("Invalid Matrix!")?

            }

        }

    }
    
}