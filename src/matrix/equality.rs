use crate::Matrix;

impl<T: std::cmp::PartialEq> PartialEq for Matrix<T> {

    fn eq(&self, b: &Self) -> bool {

        self.0.len() == b.0.len() && (0..self.0.len()).into_iter().all(|i| self.0[i] == b.0[i])

    }
    
}

impl<T: std::cmp::PartialEq> Eq for Matrix<T> {}
