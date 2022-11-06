use crate::{Integer, Bit};
use std::ops::BitAnd;

impl BitAnd for Integer {
    type Output = Self;
    fn bitand(self, b: Self) -> Self::Output {
        &self & &b
    }
}

impl BitAnd for &Integer {

    type Output = Integer;

    fn bitand(self, b: Self) -> Integer {

        let (w,x,y,z) = if self.0.len() > b.0.len() {

            let d = self.0.len() - b.0.len();

            (&self.0[0..d], &b.0[0], &self.0[d..], &b.0)

        } else {

            let d = b.0.len() - self.0.len();

            (&b.0[0..d], &self.0[0], &b.0[d..], &self.0)

        };

        let and_bits = vec![
            w.iter().map(|o| o & x).collect::<Vec<Bit>>(),
            y.iter().enumerate().map(|(i,o)| o & z[i]).collect()
        ].concat();

        Integer(and_bits)

    }

}
