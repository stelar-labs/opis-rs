use crate::{Bit, Int};
use std::error::Error;

// EQUALITY
impl PartialEq for Int {
    fn eq(&self, b: &Self) -> bool {
        self.bits == b.bits
    }
}

impl Eq for Int {}

// COMPARISON
use std::cmp::Ordering;

impl Ord for Int {

    fn cmp(&self, b: &Self) -> Ordering {

        let mut result = Ordering::Equal;

        if self.bits[0] == Bit::Zero && b.bits[0] == Bit::One {

            result = Ordering::Greater

        } else if self.bits[0] == Bit::One && b.bits[0] == Bit::Zero {

            result = Ordering::Less

        } else {
            
            let precision = if self.bits.len() > b.bits.len() {
                self.bits.len()
            } else {
                b.bits.len()
            };

            let a_diff = precision - self.bits.len();

            let b_diff = precision - b.bits.len();
            
            for i in 1..precision {

                let a_bit = if i >= a_diff {
                    self.bits[i - a_diff]
                } else {
                    self.bits[0]
                };
        
                let b_bit = if i >= b_diff {
                    b.bits[i - b_diff]
                } else {
                    b.bits[0]
                };

                match (a_bit, b_bit) {
                    (Bit::Zero, Bit::One) => { result = Ordering::Less; break; },
                    (Bit::One, Bit::Zero) => { result = Ordering::Greater; break; },
                    _ => ()
                }
            }
        }
        result
    }
}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::fmt;
// NOT
use std::ops::Not;

impl Not for Int {
    type Output = Self; fn not(self) -> Self::Output {
        not(&self)
    }
}

impl Not for &Int {
    type Output = Int; fn not(self) -> Int {
        not(self)
    } 
}

fn not(a: &Int) -> Int {
    Int { bits: a.bits.iter().map(|x| !x).collect() }
}

// AND
use std::ops::BitAnd;

impl BitAnd for Int {
    type Output = Self;
    fn bitand(self, b: Self) -> Self::Output {
        and(&self, &b)
    }
}

impl BitAnd for &Int {
    type Output = Int;
    fn bitand(self, b: Self) -> Int {
        and(self, b)
    }
}

fn and(a: &Int, b: &Int) -> Int {

    let precision = if a.bits.len() > b.bits.len() {
            a.bits.len()
        } else {
            b.bits.len()
        };

    let mut and_bits = Vec::new();

    for i in 0..precision {

        let a_bit = if i > a.bits.len() {
            a.bits[0]
        } else {
            a.bits[a.bits.len() - i]
        };

        let b_bit = if i > b.bits.len() {
            b.bits[0]
        } else {
            b.bits[b.bits.len() - i]
        };
        
        and_bits[precision - i] = a_bit & b_bit

    }

    Int { bits: and_bits }

}

// OR
use std::ops::BitOr;

impl BitOr for Int {
    type Output = Self;
    fn bitor(self, b: Self) -> Self::Output {
        or(&self, &b)
    }
}

impl BitOr for &Int {
    type Output = Int;
    fn bitor(self, b: Self) -> Int {
        or(self, b)
    }
}

fn or(a: &Int, b: &Int) -> Int {

    let precision = if a.bits.len() > b.bits.len() {
            a.bits.len()
        } else {
            b.bits.len()
        };

    let mut or_bits = vec![Bit::Zero; precision];

    for i in 0..precision {

        let a_bit = if i > a.bits.len() {
            a.bits[0]
        } else {
            a.bits[a.bits.len() - i]
        };

        let b_bit = if i > b.bits.len() {
            b.bits[0]
        } else {
            b.bits[b.bits.len() - i]
        };
        
        or_bits[precision - i] = a_bit | b_bit

    }

    Int { bits: or_bits }

}

// XOR
use std::ops::BitXor;

impl BitXor for Int {
    type Output = Self;
    fn bitxor(self, b: Self) -> Self::Output {
        xor(&self, &b)
    }
}

impl BitXor for &Int {
    type Output = Int;
    fn bitxor(self, b: Self) -> Int {
        xor(self, b)
    } 
}

fn xor(a: &Int, b: &Int) -> Int {

    let precision = if a.bits.len() > b.bits.len() {
            a.bits.len()
        } else {
            b.bits.len()
        };

    let mut xor_bits = vec![Bit::Zero; precision];

    for i in 0..precision {

        let a_bit = if i > a.bits.len() {
            a.bits[0]
        } else {
            a.bits[a.bits.len() - i]
        };

        let b_bit = if i > b.bits.len() {
            b.bits[0]
        } else {
            b.bits[b.bits.len() - i]
        };

        xor_bits[precision - i] = a_bit ^ b_bit

    }

    Int { bits: xor_bits }

}

// LEFT SHIFT 
use std::ops::{Shl, ShlAssign};

impl Shl<usize> for Int {
    type Output = Self;
    fn shl(self, shifts: usize) -> Self::Output {
        shift_left(&self, shifts)
    }
}

impl Shl<usize> for &Int {
    type Output = Int;
    fn shl(self, shifts: usize) -> Int {
        shift_left(self, shifts)
    }
}

impl ShlAssign<usize> for Int {
    fn shl_assign(&mut self, shifts: usize) {
        * self = shift_left(self, shifts)
    }
}

fn shift_left(a: &Int, shifts: usize) -> Int {
    Int { bits: [a.bits.to_vec(), vec![Bit::Zero; shifts]].concat() }
}

// RIGHT SHIFT
use std::ops::{Shr, ShrAssign};

impl Shr<usize> for Int {
    type Output = Self;
    fn shr(self, shifts: usize) -> Self::Output {
        shift_right(&self, shifts)
    }
}

impl Shr<usize> for &Int {
    type Output = Int;
    fn shr(self, shifts: usize) -> Int {
        shift_right(self, shifts)
    }
}

impl ShrAssign<usize> for Int {
    fn shr_assign(&mut self, shifts: usize) {
        *self = shift_right(self, shifts)
    }
}

fn shift_right(a: &Int, shifts: usize) -> Int {
    if shifts <= a.bits.len() - 2 {
        Int { bits: a.bits[0..a.bits.len() - shifts].to_vec() }
    } else {
        Int { bits: vec![a.bits[0];2] }
    }
}

// ADDITION
use std::ops::{Add, AddAssign};

impl Add for Int {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        adder(&self, &b)
    }
}

impl Add for &Int {
    type Output = Int;
    fn add(self, b: Self) -> Int {
        adder(self, b)
    }
}

impl AddAssign for Int {
    fn add_assign(&mut self, b: Self) {
        *self = adder(&self, &b)
    }
}

impl AddAssign<&Int> for Int {
    fn add_assign(&mut self, b: &Int) {
        *self = adder(&self, b)
    }
}

fn adder(a: &Int, b: &Int) -> Int {
    
    let precision =
        if a.bits.len() > b.bits.len() {
            a.bits.len() + 1
        } else {
            b.bits.len() + 1
        };

    let (bits, _) = (1..=precision)
        .into_iter()
        .fold(
            (vec![Bit::Zero; precision], Bit::Zero),
            |(mut bits, mut carry), x| {
            
                let a_bit = if x > a.bits.len() {
                    a.bits[0]
                } else {
                    a.bits[a.bits.len() - x]
                };

                let b_bit = if x > b.bits.len() {
                    b.bits[0]
                } else {
                    b.bits[b.bits.len() - x]
                };

                let (carry_1, sum_1) = carry + a_bit;

                let (carry_2, sum_2) = b_bit + sum_1;

                carry = carry_1 ^ carry_2;

                bits[precision - x] = sum_2;

                (bits, carry)

            }
        );
        
    let mut result = Int { bits };

    result.clean();

    result

}

// SUBTRACTION
use std::ops::{Sub, SubAssign};

impl Sub for Int {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        self + b.negate()
    }
}

impl Sub for &Int {
    type Output = Int;
    fn sub(self, b: Self) -> Int {
        self + &b.negate()
    }
}

impl SubAssign for Int {
    fn sub_assign(&mut self, b: Self) {
        *self = self.clone() + b.negate()
    }
}

impl SubAssign<&Int> for Int {
    fn sub_assign(&mut self, b: &Int) {
        *self = self.clone() + b.negate()
    }
}

// MULTIPLICATION

use std::ops::{Mul, MulAssign};

impl Mul for Int {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        multiplier(&self, &b)
    }
}

impl Mul for &Int {
    type Output = Int;
    fn mul(self, b: Self) -> Int {
        multiplier(self, b)
    }
}

impl MulAssign for Int {
    fn mul_assign(&mut self, b: Self) {
        *self = multiplier(self, &b)
    }
}

impl MulAssign<&Int> for Int {
    fn mul_assign(&mut self, b: &Int) {
        *self = multiplier(self, b)
    }
}

fn multiplier(a: &Int, b: &Int) -> Int {

    let precision = if a.bits.len() > b.bits.len() {
            a.bits.len() * 2
        } else {
            b.bits.len() * 2
        };

    (0..precision)
        .into_iter()
        .fold(Int::zero(), |acc, x| {
            
            let b_bit =
                if x > b.bits.len() - 1 {
                    b.bits[0]
                } else {
                    b.bits[b.bits.len() - x - 1]
                };

            if b_bit == Bit::One {
                acc + (a << x)
            } else {
                acc
            }

        })
}

// DIVISION
use std::ops::Div;

impl Div for Int {

    type Output = Result<Self, Box<dyn Error>>;
    
    fn div(self, b: Self) -> Result<Self, Box<dyn Error>> {
        
        if self == Int::zero() {

            Ok(Int::zero())

        } else if b == Int::zero() {

            Err("a/0 is undefined!")?

        } else {

            let a_pos =
                if self.bits[0] == Bit::Zero {
                    self.clone()
                } else {
                    self.negate()
                };

            let b_pos =
                if b.bits[0] == Bit::Zero {
                    b.clone()
                } else {
                    b.negate()
                };

            let (quotient, _) = div(&a_pos, &b_pos);

            match (self.bits[0], b.bits[0]) {
                (Bit::Zero, Bit::Zero) => Ok(quotient),
                (Bit::One, Bit::One) => Ok(quotient),
                (Bit::Zero, Bit::One) => Ok(quotient.negate()),
                (Bit::One, Bit::Zero) => Ok(quotient.negate())
            }
        }
    }
}

impl Div for &Int {
    type Output = Result<Int, Box<dyn Error>>;
    fn div(self, b: Self) -> Result<Int, Box<dyn Error>> {
        self.clone() / b.clone()
    }
}

pub fn div(numerator: &Int, denominator: &Int) -> (Int, Int) {

    let mut quotient = Int::zero();

    let mut remainder = Int { bits: numerator.bits[..1].to_vec() };

    numerator.bits
        .iter()
        .skip(1)
        .for_each(|x| {

            remainder.bits.push(*x);

            if &remainder > denominator {

                quotient.bits.push(Bit::One);
                
                remainder = &remainder - denominator;
                
            } else if &remainder == denominator {

                quotient.bits.push(Bit::One);

                remainder = Int::zero();

            } else {

                quotient.bits.push(Bit::Zero);

            };

        });
    
    quotient.clean();

    remainder.clean();

    (quotient, remainder)

}


// REMAINDER
use std::ops::Rem;

impl Rem for Int {

    type Output = Result<Self, Box<dyn Error>>;
    
    fn rem(self, b: Self) -> Result<Self, Box<dyn Error>> {
        
        if self == Int::zero() {
            
            Ok(Int::zero())
        
        } else if b == Int::zero() {
            
            Err("a/0 is undefined!")?
        
        } else {

            let a_pos =
                if self.bits[0] == Bit::Zero {
                    self.clone()
                } else {
                    self.negate()
                };

            let b_pos =
                if b.bits[0] == Bit::Zero {
                    b
                } else {
                    b.negate()
                };
    
            let (_, remainder) = div(&a_pos, &b_pos);
    
            if self.bits[0] == Bit::One {
                
                Ok(remainder.negate())
            
            } else {

                Ok(remainder)
            }

        }

    }

}

impl Rem for &Int {

    type Output = Result<Int, Box<dyn Error>>;

    fn rem(self, b: Self) -> Result<Int, Box<dyn Error>> {
        
        self.clone() % b.clone()

    }

}

impl Int {

    // EXPONENTIATION
    pub fn pow(&self, e: &Int) -> Result<Int, Box<dyn Error>> {
        
        if self == &Int::zero() {
            
            Ok(Int::zero())

        } else if e == &Int::zero() {
            
            Ok(Int::one())

        } else if e.bits[0] == Bit::One {
            
            Err("Non Integer result for negative exponent!")?

        } else {
            
            Ok(
                e.bits
                    .iter()
                    .skip(2)
                    .fold(
                        self.clone(),
                        |acc, x| {
                            let mut result = &acc * &acc;
                            if x == &Bit::One {
                                result *= self
                            }
                            result
                        }
                    )
            )

        }

    }

    // MODULO
    pub fn modulo(&self, modulus: &Int) -> Result<Self, Box<dyn Error>> {

        if self == &Int::zero() {

            Ok(Int::zero())

        } else if modulus == &Int::zero() {

            Err("a/0 is undefined!")?

        } else {

            let mut result = (self % modulus)?;

            while result < Int::zero() {
                result += result.clone()

            }

            Ok(result)

        }

    }

    // BINARY CONVERSION
    pub fn from_bin(str: &str) -> Result<Self, Box<dyn Error>> {

        if str.len() > 0 {
            
            let mut bin_str: Vec<_> = str.split("").collect();
            
            bin_str.retain(|&x| x != "");

            let mut bits = Vec::new();

            for bit in bin_str {
                match bit {
                    "0" => bits.push(Bit::Zero),
                    "1" => bits.push(Bit::One),
                    _ => Err("Internal error!")?
                }
            }

            let mut result = Int { bits };

            result.clean();

            Ok(result)

        } else {

            Err("Internal error!")?

        }

    }

    pub fn to_bin(&self) -> String {
        self.bits
            .iter()
            .fold(
                String::new(),
                |mut acc, x| {
                    match x {
                        Bit::Zero => acc.push('0'),
                        Bit::One => acc.push('1')
                    }
                    acc
                }
            )
    }

    pub fn to_ext_bits(&self, length: usize) -> Vec<Bit> {

        let ext =
            if self.bits.len() < length {
                length - self.bits.len()
            } else {
                0
            };
        
        [vec![self.bits[0]; ext], self.bits.clone()].concat()

    }

    // DECIMAL CONVERSION
    pub fn from_dec(str: &str) -> Result<Self, Box<dyn Error>> {

        if str.len() > 0 {
            
            let (first, last) = str.split_at(1);
            
            let (dec_str, sign) =
                match first {
                    "-" => (last, true),
                    _ => (str, false)
                };

            let mut bits: Vec<Bit> = Vec::new();

            let mut i_str = dec_str.to_string();

            while i_str != "0".to_string() {

                let (s_half, rem) = half(&i_str);

                i_str = s_half;

                match rem {
                    0 => bits.push(Bit::Zero),
                    1 => bits.push(Bit::One),
                    _ => Err("Internal error!")?
                }

            }

            bits.push(Bit::Zero);

            bits.reverse();
            
            if bits.is_empty() {
                    
                Ok(Int::zero())

            } else {
                
                if sign {
                    
                    Ok(Int { bits }.negate())

                } else {

                    Ok(Int { bits })

                }

            }
            
        } else {

            Err("Internal error!")?
        
        }

    }

    pub fn to_dec(&self) -> String {
        
        if self.bits[0] == Bit::One {
            bits_to_dec_string(&self.negate().bits[1..])
        } else {
            bits_to_dec_string(&self.bits[1..])
        }

    }

    // HEXADECIMAL CONVERSION
    pub fn to_hex(&self) -> String {

        let int_bytes: Vec<u8> = self.into();
        
        int_bytes
            .iter()
            .fold(
                String::new(),
                |mut acc, x| {
                    acc.push_str(&format!("{:02x}", x));
                    acc
                }
            )

    }

    pub fn from_hex(str: &str) -> Result<Self, Box<dyn Error>> {

        if str.len() > 0 {

            let mut bin_str = String::new();
            
            let mut str_vec: Vec<_> = str.split("").collect();
                    
            str_vec.retain(|&x| x != "");

            for char in str_vec {

                let char_byte = u8::from_str_radix(char, 16)?;

                let char_bin = format!("{:04b}", char_byte);

                bin_str.push_str(&char_bin)

            }

            Int::from_bin(&bin_str)

        } else {
            
            Err("Internal error!")?
        
        }

    }

    pub fn to_ext_bytes(&self, length: usize) -> Vec<u8> {
        Int { bits: self.to_ext_bits(length * 8) }.into()
    }

    pub fn negate(&self) -> Int {
        !self + Int::one()
    }

    pub fn lfsr(&self, iterations: usize) -> Result<Self, Box<dyn Error>> {

        let mut result = self.clone();
        
        let mut fibs: Vec<usize> = vec![1,2];

        let mut next_fib = fibs[fibs.len() - 2] + fibs[fibs.len() - 1];

        while next_fib < self.bits.len() {

            fibs.push(next_fib);

            next_fib = fibs[fibs.len() - 2] + fibs[fibs.len() - 1];
            
        }

        for _ in 0..iterations {

            let output = fibs
                .iter()
                .skip(1)
                .fold(
                    result.bits[result.bits.len() - 1],
                    |acc, x| acc ^ result.bits[result.bits.len() - x]
                );

            result.bits.pop();

            result.bits = [vec![output], result.bits].concat();

        }

        Ok(result)

    }

    pub fn zero() -> Self {
        Int { bits: vec![Bit::Zero, Bit::Zero] }
    }

    pub fn one() -> Self {
        Int { bits: vec![Bit::Zero, Bit::One] }
    }

    pub fn two() -> Self {
        Int { bits: vec![Bit::Zero, Bit::One, Bit::Zero] }
    }

    pub fn three() -> Self {
        Int { bits: vec![Bit::Zero, Bit::One, Bit::One] }
    }

    pub fn clean(&mut self) {
        while self.bits.len() > 2 && self.bits[0] == self.bits[1] {
            self.bits.remove(0);
        }
    }

    pub fn bits(&self) -> Vec<Bit> {
        self.bits.clone()
    }

}

impl fmt::Debug for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Int")
        .field("bits", &self.bits)
        .finish()
    }
}

impl From<&[Bit]> for Int {
    fn from(bits: &[Bit]) -> Self {
        Int { bits: bits.to_vec() }
    }
}

// BYTE CONVERSION
// impl TryFrom<Vec<u8>> for Int {
//     fn try_from(value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
//         Ok(Int::from(&value[..]))
//     }

//     type Error = Box<dyn Error>;

// }

impl From<Vec<u8>> for Int {
    fn from(value: Vec<u8>) -> Self {
        Int::from(&value[..])
    }
}

impl From<&[u8]> for Int {
    fn from(bytes: &[u8]) -> Self {
        Int::from_bin(
            &bytes
                .iter()
                .fold(String::new(), |acc, x| format!("{}{:08b}", acc, x))
        ).unwrap()
    }
}

impl Into<Vec<u8>> for Int {
    fn into(self) -> Vec<u8> {
        into_bytes(&self.bits)
    }
}

impl Into<Vec<u8>> for &Int {
    fn into(self) -> Vec<u8> {
        into_bytes(&self.bits)
    }
}
 
fn into_bytes(bits: &[Bit]) -> Vec<u8> {
    
    let mut result = Vec::new();
    
    let rem = bits.len() % 8;

    let fill = 8 - rem;

    let byte_count = 
    
        if rem == 0 {

            bits.len() / 8

        } else {
            
            (bits.len() / 8) + 1
        
        };

    

    for byte_index in 0..byte_count {

        let mut byte_bits = Vec::new();

        let mut i =
        
            if byte_index == 0 {

                if rem != 0 {

                    byte_bits = vec![bits[0]; fill]

                };

                0

            } else {

                (byte_index * 8) - fill

            };

        while byte_bits.len() < 8 {

            byte_bits.push(bits[i]);

            i += 1;

        }

        let byte_int = Int { bits: byte_bits };

        let byte = u8::from_str_radix(&byte_int.to_bin(), 2).unwrap();

        result.push(byte)

    }
    
    result

}

fn bits_to_dec_string(bits: &[Bit]) -> String {

    bits.iter().fold("0".to_string(), |mut acc, x| { 
        acc = double(&acc);
        if x == &Bit::One {
            acc = add_one(&acc);
        }
        acc
    })

}

fn double(s: &str) -> String {

    let mut split: Vec<_> = s.split("").collect();
    
    split.retain(|&x| x != "");

    split.reverse();

    let mut carry: u8 = 0;

    let mut current_number: Vec<u8> = Vec::new();

    for i in split {

        let mut res: u8 = carry;

        let n: u8 = u8::from_str_radix(&i, 10).unwrap();

        let d: u8 = n * 2;

        if d > 9 {

            res += d - 10;

            carry = 1;

        } else {

            res += d;

            carry = 0;

        }

        current_number.push(res)

    }

    if carry == 1 {

        current_number.push(1)
    
    }

    current_number.reverse();

    let num_str: String = current_number
        .iter()
        .fold(
            String::new(),
            |acc, x|
            format!("{}{}", acc, x)
        );

    num_str

}

fn add_one(s: &str) -> String {

    let mut split: Vec<_> = s.split("").collect();
    
    split.retain(|&x| x != "");

    split.reverse();

    let mut carry: u8 = 1;

    let mut current_number: Vec<u8> = Vec::new();

    for i in split {

        let n: u8 = u8::from_str_radix(&i, 10).unwrap();

        let mut sum: u8 = carry + n;

        if sum == 10 {

            sum = 0;

            carry = 1;

        } else {

            carry = 0;

        }

        current_number.push(sum)

    }

    if carry == 1 {

        current_number.push(1)
    
    }

    current_number.reverse();

    let num_str: String = current_number
    .iter()
    .fold(
        String::new(),
        |acc, x|
        format!("{}{}", acc, x)
    );

    num_str

}

fn half(s: &str) -> (String, u8) {

    let mut split: Vec<_> = s.split("").collect();
    
    split.retain(|&x| x != "");

    let split_size = split.len();

    let mut res: String = String::new();

    let mut rem: u8 = 0;

    for i in split {

        let n_str = format!("{}{}", rem, i);

        let n = u8::from_str_radix(&n_str, 10).unwrap();

        let d = n/2;

        if res == String::new() {

            if split_size == 1 {

                res.push_str(&d.to_string())

            } else {
            
                if d != 0 {
                    
                    res.push_str(&d.to_string())
                
                }

            }

        } else {

            res.push_str(&d.to_string())

        }
        
        rem = n%2;

    }

    (res, rem)

}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Int::one() + Int::one(), Int::two());
    }
    #[test]
    fn test_sub() {
        assert_eq!(Int::three() - Int::one(), Int::two());
    }
    #[test]
    fn test_mul() {
        assert_eq!(Int::two() * Int::one(), Int::two());
    }
    #[test]
    fn test_div() {
        assert_eq!((Int::three() / Int::two()).unwrap(), Int::one());
    }
    #[test]
    fn test_rem() {
        assert_eq!((Int::three() % Int::two()).unwrap(), Int::one());
    }
    #[test]
    fn test_shl() {
        assert_eq!(Int::one() << 1, Int::two());
    }
    #[test]
    fn test_shr() {
        assert_eq!(Int::two() >> 1, Int::one());
    }
    #[test]
    fn test_from_bin() {
        assert_eq!(Int::from_bin("010").unwrap(), Int::two());
    }
    #[test]
    fn test_to_bin() {
        assert_eq!(Int::two().to_bin(), "010");
    }
    #[test]
    fn test_from_dec() {
        assert_eq!(Int::from_dec("3").unwrap(), Int::three());
    }
    #[test]
    fn test_to_dec() {
        assert_eq!(Int::three().to_dec(), "3");
    }
    #[test]
    fn test_from_hex() {
        assert_eq!(Int::from_hex("01").unwrap(), Int::one());
    }
    #[test]
    fn test_to_hex() {
        assert_eq!(Int::one().to_hex(), "01");
    }
    #[test]
    fn test_from() {
        assert_eq!(Int::from(&[2][..]), Int::two());
    }
    #[test]
    fn test_into() {
        let int_bytes: Vec<u8> = Int::two().into();
        assert_eq!(int_bytes, vec![2]);
    }
    #[test]
    fn text_exp() {
        assert_eq!(
            Int::two().pow(&Int::two()).unwrap(),
            Int { bits: vec![Bit::Zero, Bit::One, Bit::Zero, Bit::Zero] }
        )
    }
    #[test]
    fn test_lfsr() {
        assert_eq!(Int::one().lfsr(1).unwrap(), Int {bits: vec![Bit::Zero ^ Bit::One, Bit::Zero]})
    }
    #[test]
    fn test_eq() {
        assert!(Int::two() == Int::two());
    }

    #[test]
    fn test_gt() {
        assert!(Int::three() > Int::one());
    }

    #[test]
    fn test_lt() {
        assert!(Int::one() < Int::two());
    }
}