use crate::Fraction;

impl Fraction {
    pub fn reduce(&mut self) {
        let (gcd, _, _) = self.0.ext_gcd(&self.1);
        self.0 = (&self.0 / &gcd).unwrap();
        self.1 = (&self.1 / &gcd).unwrap();
    }
}
