use crate::{Integer, Bit};
use std::{ops::Rem, error::Error};
use super::divider;

impl Rem for Integer {
    type Output = Result<Self, Box<dyn Error>>;
    fn rem(self, b: Self) -> Result<Self, Box<dyn Error>> {
        &self % &b
    }
}

impl Rem for &Integer {

    type Output = Result<Integer, Box<dyn Error>>;

    fn rem(self, b: Self) -> Result<Integer, Box<dyn Error>> {
        
        if self == &Integer::zero() {
            
            Ok(Integer::zero())
        
        } else if b == &Integer::zero() {
            
            Err("a%0 is undefined!")?
        
        } else {

            let a_pos =
                if self.0[0] == Bit::Zero {
                    self.clone()
                } else {
                    self.negate()
                };

            let b_pos =
                if b.0[0] == Bit::Zero {
                    b.clone()
                } else {
                    b.negate()
                };
    
            let (_, remainder) = divider::run(&a_pos, &b_pos);
    
            if self.0[0] == Bit::One {
                
                Ok(remainder.negate())
            
            } else {

                Ok(remainder)
            }

        }

    }

}

impl Rem<&u8> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn rem(self, b: &u8) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self % &b_int
    }
}

impl Rem<&u16> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn rem(self, b: &u16) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self % &b_int
    }
}

impl Rem<&u32> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn rem(self, b: &u32) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self % &b_int
    }
}

impl Rem<&u64> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn rem(self, b: &u64) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self % &b_int
    }
}

impl Rem<&u128> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn rem(self, b: &u128) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self % &b_int
    }
}

impl Rem<&usize> for &Integer {
    type Output = Result<Integer, Box<dyn Error>>;
    fn rem(self, b: &usize) -> Result<Integer, Box<dyn Error>> {
        let b_int: Integer = b.into();
        self % &b_int
    }
}
