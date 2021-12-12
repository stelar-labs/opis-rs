
use std::error::Error;
use std::fmt;

mod base10;
mod base16;

mod adder;
mod subtractor;
mod multiplier;
mod divisor;
mod exponentiation;
mod modular_inverse;
mod comparison;

#[derive(Clone, Debug)]
pub struct Int {
    pub bits: Vec<u8>,
    pub negative: bool
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opis Error: {}", self.0)
    }
}

impl Error for CustomError {}

impl Int {

    pub fn zero() -> Self { Int { bits: vec![0], negative: false } }

    pub fn one() -> Self { Int { bits: vec![1], negative: false } }

    pub fn add(mut self, n: &Int) -> Self {

        let mut b = n.to_owned();

        let mut res = Int::zero();

        if !self.negative && !b.negative {

            res.bits = adder::run(self.bits, b.bits)

        } else if self.negative && b.negative {

            res.bits = adder::run(self.bits, b.bits);

            res.negative = true
            
        } else if self.negative {

            self.negative = false;

            res = b.sub(&self)

        } else {

            b.negative = false;

            res = self.sub(&b)

        }

        res

    }

    pub fn sub(self, n: &Int) -> Self {

        let mut b = n.to_owned();

        let mut res = Int::zero();

        if !self.negative && !b.negative {

            if self.to_owned().is_greater(&b) {

                res.bits = subtractor::run(self.bits, b.bits);

            } else {

                res.bits = subtractor::run(b.bits, self.bits);

                res.negative = true

            }
        
        } else if self.negative && b.negative {

            b.negative = false;

            res = self.add(&b)

        } else if self.negative {

            b.negative = true;

            res = self.add(&b)

        } else {

            b.negative = false;

            res = self.add(&b)

        }

        res

    }

    pub fn mul(self, b: &Int) -> Self {

        let mut res = Int::zero();

        if self.bits != vec![0] && b.bits != vec![0] {
            
            res.bits = multiplier::run(self.bits, b.to_owned().bits);

            match(self.negative, b.negative) {
                (false, true) => res.negative = true,
                (true, false) => res.negative = true,
                _ => ()
            }

        }

        res

    }

    pub fn div(self, b: &Int) -> Result<Self, Box<dyn Error>> {

        let mut res = Int::zero();

        if self.bits == vec![0] {

            Ok(res)

        } else if b.bits == vec![0] {

            Err(Box::new(CustomError("a/0 is undefined! ".into())))
    
        } else {

            let (q, _) = divisor::run(self.to_owned(), b);
            
            res = q;

            match(self.negative, b.negative) {
                (false, true) => res.negative = true,
                (true, false) => res.negative = true,
                _ => ()
            }

            Ok(res)

        }

    }

    pub fn rem(self, b: &Int) -> Result<Self, Box<dyn Error>> {

        if self.bits == vec![0] {

            Ok(Int::zero())

        } else if b.bits == vec![0] {

            Err(Box::new(CustomError("a/0 is undefined! ".into())))
    
        } else {

            let (_, r) = divisor::run(self.to_owned(), b);

            let mut res = r;

            if self.negative {

                res.negative = true

            }

            Ok(res)

        }   

    }

    pub fn modulo(self, b: &Int) -> Result<Self, Box<dyn Error>> {

        if self.bits == vec![0] {

            Ok(Int::zero())

        } else if b.bits == vec![0] {

            Err(Box::new(CustomError("a/0 is undefined! ".into())))
    
        } else {

            let r = self.rem(&b)?;

            if r.negative {

                let r_plus_b = r.add(b);

                let res = r_plus_b.rem(b).unwrap();

                Ok(res)

            } else {

                Ok(r)

            }

        }   

    }

    pub fn pow(self, b: &Int) -> Self {

        exponentiation::run(self, b)

    }

    pub fn mod_inv(self, m: &Int) -> Self {

        if self.bits == vec![0] || m.bits == vec![0] {

            Int::zero()

        } else {

            modular_inverse::run(self, m)

        }

    }

    pub fn from_str(s: &str, r: u8) -> Result<Self, Box<dyn Error>> {

        match r {

            2 => {

                let mut split: Vec<_> = s.split("").collect();
    
                split.retain(|&x| x != "");

                let bits = split
                    .iter()
                    .map(|&x| u8::from_str_radix(x, 10).unwrap())
                    .collect();

                let res = Int {
                    bits: bits,
                    negative: false
                };

                Ok(res)

            },

            10 => {

                let b = base10::from(s)?;

                let res = Int {
                    bits: b,
                    negative: false
                };

                Ok(res)

            },

            16 => {

                let b = base16::from(s)?;

                let res = Int {
                    bits: b,
                    negative: false
                };

                Ok(res)

            },

            _ => Err(Box::new(CustomError("unsupported radix!".into())))

        }

    }

    pub fn to_str(self, r: u8) -> Result<String, Box<dyn Error>> {
        
        match r {
            
            2 => {

                let mut res: String = String::with_capacity(self.bits.len());
    
                for bit in self.bits {
                    
                    res.push_str(&bit.to_string())
                
                };

                Ok(res)

            },

            10 => Ok(base10::to(self.bits)),

            16 => Ok(base16::to(self.bits)),

            _ => Err(Box::new(CustomError("unsupported radix!".into())))

        }

    }

    // comparison functions

    pub fn is_greater(self, b: &Int) -> bool {

        if &compare(self, b.to_owned()) == "greater" {
            
            true
        
        } else {
            
            false
        
        }

    }

    pub fn is_less(self, b: &Int) -> bool {

        if &compare(self, b.to_owned()) == "less" {

            true

        } else {

            false

        }

    }

    pub fn is_equal(self, b: &Int) -> bool {

        if &compare(self, b.to_owned()) == "equal" {

            true

        } else {

            false

        }

    }

    // bitwise functions
    pub fn not(mut self) -> Self {

        self.bits = self.bits.iter()
            .map(|x| { match x { 1 => 0, _ => 1 } })
            .collect();

        self
        
    }

    pub fn and(self, y: &Int) -> Self {

        let mut x_bits: Vec<u8> = self.bits;

        let mut y_bits: Vec<u8> = y.bits.clone();

        let mut and_bits: Vec<u8> = Vec::new();

        while x_bits.len() > 0 || y_bits.len() > 0 {

            let x_bit = match x_bits.pop() { Some(r) => r, None => 0 };
    
            let y_bit = match y_bits.pop() { Some(r) => r, None => 0 };

            let and_bit = match (x_bit, y_bit) { (1, 1) => 1, _ => 0 };

            and_bits.push(and_bit)

        }

        and_bits.reverse();

        let res = Int { bits: and_bits, negative: false };

        res

    }

    pub fn or(self, y: &Int) -> Self {
        
        let mut x_bits: Vec<u8> = self.bits;

        let mut y_bits: Vec<u8> = y.bits.clone();

        let mut or_bits: Vec<u8> = Vec::new();

        while x_bits.len() > 0 || y_bits.len() > 0 {

            let x_bit = match x_bits.pop() { Some(r) => r, None => 0 };
    
            let y_bit = match y_bits.pop() { Some(r) => r, None => 0 };

            let or_bit = match (x_bit, y_bit) { (0, 0) => 0, _ => 1 };

            or_bits.push(or_bit)

        }

        or_bits.reverse();

        let res = Int { bits: or_bits, negative: false };

        res

    }

    pub fn xor(self, y: &Int) -> Self {
        
        let mut x_bits: Vec<u8> = self.bits;

        let mut y_bits: Vec<u8> = y.bits.clone();

        let mut xor_bits: Vec<u8> = Vec::new();

        while x_bits.len() > 0 || y_bits.len() > 0 {

            let x_bit = match x_bits.pop() { Some(r) => r, None => 0 };
    
            let y_bit = match y_bits.pop() { Some(r) => r, None => 0 };

            let xor_bit = match (x_bit, y_bit) {
                (0, 1) => 1,
                (1, 0) => 1,
                _ => 0
            };

            xor_bits.push(xor_bit)

        }

        xor_bits.reverse();

        let res = Int { bits: xor_bits, negative: false };

        res

    }

}

fn compare(a: Int, b: Int) -> String {

    if !a.negative && b.negative {
            
        "greater".to_string()
    
    } else if a.negative && !b.negative {

        "less".to_string()

    } else if !a.negative && !b.negative {

        comparison::run(a, b)

    } else {

        match &comparison::run(a, b)[..] {
            "greater" => "less".to_string(),
            "less" => "greater".to_string(),
            _ => "equal".to_string()
        }

    }


}
