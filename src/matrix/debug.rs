use std::fmt;
use crate::Matrix;

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            write!(f, "[")?;
            for element in row {
                write!(f, "{:?}, ", element)?;
            }
            write!(f, "], \n")?;
        }
        Ok(())
    }
}