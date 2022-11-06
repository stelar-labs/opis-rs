use crate::{Integer, Fraction};


impl From<&Integer> for Fraction {
    fn from(value: &Integer) -> Self {
        Fraction(value.clone(), Integer::one())
    }
}

impl From<&u8> for Fraction {
    fn from(value: &u8) -> Self {
        let v_int: Integer = value.into();
        Fraction(v_int, Integer::one())
    }
    
}

impl From<&u16> for Fraction {
    fn from(value: &u16) -> Self {
        let v_int: Integer = value.into();
        Fraction(v_int, Integer::one())
    }
}

impl From<&u32> for Fraction {
    fn from(value: &u32) -> Self {
        let v_int: Integer = value.into();
        Fraction(v_int, Integer::one())
    }
}

impl From<&u64> for Fraction {
    fn from(value: &u64) -> Self {
        let v_int: Integer = value.into();
        Fraction(v_int, Integer::one())
    }
}

impl From<&u128> for Fraction {
    fn from(value: &u128) -> Self {
        let v_int: Integer = value.into();
        Fraction(v_int, Integer::one())
    }
}

impl From<&usize> for Fraction {
    fn from(value: &usize) -> Self {
        let v_int: Integer = value.into();
        Fraction(v_int, Integer::one())
    }
}
