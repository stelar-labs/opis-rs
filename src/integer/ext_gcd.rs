use crate::Integer;


impl Integer {

    pub fn ext_gcd(&self, b: &Integer) -> (Integer, Integer, Integer) {

        if self == &Integer::zero() {

            (b.clone(), Integer::zero(), Integer::one())

        } else if b == &Integer::zero() {

            (self.clone(), Integer::one(), Integer::zero())
            
        } else {
            
            let (gcd, s, t) = (b % self).unwrap().ext_gcd(self);
            
            (gcd, t - &(b / self).unwrap() * &s, s)

        }

    }
    
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_int_ext_gcd_0() {

        let a = Integer::from_dec("25").unwrap();

        let b = Integer::from_dec("10").unwrap();

        let gcd = Integer::from_dec("5").unwrap();

        let s = Integer::from_dec("1").unwrap();

        let t = Integer::from_dec("-2").unwrap();

        assert_eq!(a.ext_gcd(&b), (gcd, s, t));
    }

    #[test]
    fn test_int_ext_gcd_1() {

        let a = Integer::from_dec("10").unwrap();

        let b = Integer::from_dec("6").unwrap();

        let gcd = Integer::from_dec("2").unwrap();

        let s = Integer::from_dec("-1").unwrap();

        let t = Integer::from_dec("2").unwrap();

        assert_eq!(a.ext_gcd(&b), (gcd, s, t));
    }

}