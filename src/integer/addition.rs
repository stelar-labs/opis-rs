
// use crate::{Integer, Bit};
// use std::ops::{Add, AddAssign};

// impl Add for Integer {
//     type Output = Self;
//     fn add(self, b: Self) -> Self {
//         &self + &b
//     }
// }

// impl Add for &Integer {

//     type Output = Integer;

//     fn add(self, b: Self) -> Integer {

//         if self == &Integer::zero() {
//             b.clone()
//         } else if b == &Integer::zero() {
//             self.clone()
//         } else {
//             let sum_bits = adder(&self.0, &b.0);
//             let mut result = Integer(sum_bits);
//             result.truncate();
//             result
//         }

//     }
// }

// impl Add<&u8> for &Integer {
//     type Output = Integer;
//     fn add(self, b: &u8) -> Integer {
//         let b_int: Integer = b.into();
//         self + &b_int
//     }
// }

// impl Add<&u16> for &Integer {
//     type Output = Integer;
//     fn add(self, b: &u16) -> Integer {
//         let b_int: Integer = b.into();
//         self + &b_int
//     }
// }

// impl Add<&u32> for &Integer {
//     type Output = Integer;
//     fn add(self, b: &u32) -> Integer {
//         let b_int: Integer = b.into();
//         self + &b_int
//     }
// }

// impl Add<&u64> for &Integer {
//     type Output = Integer;
//     fn add(self, b: &u64) -> Integer {
//         let b_int: Integer = b.into();
//         self + &b_int
//     }
// }

// impl Add<&u128> for &Integer {
//     type Output = Integer;
//     fn add(self, b: &u128) -> Integer {
//         let b_int: Integer = b.into();
//         self + &b_int
//     }
// }

// impl Add<&usize> for &Integer {
//     type Output = Integer;
//     fn add(self, b: &usize) -> Integer {
//         let b_int: Integer = b.into();
//         self + &b_int
//     }
// }

// impl AddAssign for Integer {
//     fn add_assign(&mut self, b: Self) {
//         *self = &self.clone() + &b
//     }
// }

// impl AddAssign<&Integer> for Integer {
//     fn add_assign(&mut self, b: &Integer) {
//         *self = &self.clone() + b
//     }
// }

// impl AddAssign<&u8> for Integer {
//     fn add_assign(&mut self, b: &u8) {
//         let b_int: Integer = b.into();
//         *self = &self.clone() + &b_int
//     }
// }

// impl AddAssign<&u16> for Integer {
//     fn add_assign(&mut self, b: &u16) {
//         let b_int: Integer = b.into();
//         *self = &self.clone() + &b_int
//     }
// }

// impl AddAssign<&u32> for Integer {
//     fn add_assign(&mut self, b: &u32) {
//         let b_int: Integer = b.into();
//         *self = &self.clone() + &b_int
//     }
// }

// impl AddAssign<&u64> for Integer {
//     fn add_assign(&mut self, b: &u64) {
//         let b_int: Integer = b.into();
//         *self = &self.clone() + &b_int
//     }
// }

// impl AddAssign<&u128> for Integer {
//     fn add_assign(&mut self, b: &u128) {
//         let b_int: Integer = b.into();
//         *self = &self.clone() + &b_int
//     }
// }

// impl AddAssign<&usize> for Integer {
//     fn add_assign(&mut self, b: &usize) {
//         let b_int: Integer = b.into();
//         *self = &self.clone() + &b_int
//     }
// }

// pub fn adder(a: &[Bit], b: &[Bit]) -> Vec<Bit> {
    
//     let precision = if a.len() > b.len() { a.len() } else { b.len() } + 1;

//     let mut a_pos = a.len() - 1;

//     let mut b_pos = b.len() - 1;

//     let mut carry = Bit::Zero;

//     (0..precision)
//         .into_iter()
//         .rev()
//         .fold(
//             vec![],
//             |sum, _|
//             {

//                 let a_bit = if a_pos == 0 { a[0] } else { let r = a[a_pos]; a_pos -= 1; r };
                
//                 let b_bit = if b_pos == 0 { b[0] } else { let r = b[b_pos]; b_pos -= 1; r };

//                 let (carry_1, sum_1) = carry + a_bit;

//                 let (carry_2, sum_2) = sum_1 + b_bit;

//                 carry = carry_1 ^ carry_2;

//                 vec![sum_2].into_iter().chain(sum.into_iter()).collect()

//             }
        
//         )

// }

use std::ops::Add;

use crate::Integer;

use super::Digit;

impl Add for Integer {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        &self + &b
    }
}

impl Add for &Integer {

    type Output = Integer;

    fn add(self, other: Self) -> Integer {

        let mut result = Vec::new();
        let mut carry = 0;

        let a_highest_bit = self.most_significant_bit();
        let b_highest_bit = other.most_significant_bit();

        // Calculate extended digit lengths
        let a_ext_digit_len = if self.count_extended_bits(a_highest_bit) == 0 {
            self.digits.len() + 1
        } else {
            self.digits.len()
        };

        let b_ext_digit_len = if other.count_extended_bits(b_highest_bit) == 0 {
            other.digits.len() + 1
        } else {
            other.digits.len()
        };

        // Cache the extension digit for both integers
        let ext_digit = if a_ext_digit_len > self.digits.len() {
            if a_highest_bit {
                Digit::MAX // Extend with MAX value if the number is negative
            } else {
                0 // Extend with 0 if the number is positive
            }
        } else {
            if b_highest_bit {
                Digit::MAX // Extend with MAX value if the number is negative
            } else {
                0 // Extend with 0 if the number is positive
            }
        };

        // Determine the maximum length to iterate over
        let max_len = a_ext_digit_len.max(b_ext_digit_len);

        // Perform the addition with carry, using cached extended digits as necessary
        for i in 0..max_len {
            let a_digit = match self.digits.get(i) {
                Some(r) => r,
                None => &ext_digit
            };
            let b_digit = match other.digits.get(i) {
                Some(r) => r,
                None => &ext_digit
            };

            let (sum, carry_out1) = a_digit.overflowing_add(*b_digit);
            let (sum, carry_out2) = sum.overflowing_add(carry);
            carry = (carry_out1 as Digit) | (carry_out2 as Digit);

            result.push(sum);
            
        }

        Integer { digits: result }

    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_int_one_plus_one_eq_two() {
        assert_eq!(Integer::one() + Integer::one(), Integer::two());
    }

    #[test]
    fn test_int_one_plus_neg_one_eq_zero() {
        assert_eq!(Integer::one() + Integer::neg_one(), Integer::zero());
    }

    // #[test]
    // fn test_int_plus_u8() {
    //     assert_eq!(&Integer::one() + &1_u8, Integer::two());
    // }

}
