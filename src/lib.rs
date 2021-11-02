
use std::error::Error;
use std::fmt;

mod base10;

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

            if &self.clone().cmp(&b)[..] == "greater" {

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

            10 => {

                let b = base10::from(s)?;

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

            _ => Err(Box::new(CustomError("unsupported radix!".into())))

        }

    }

    pub fn cmp(self, b: &Int) -> String {

        if self.bits == b.bits && self.negative == b.negative {

            "equal".to_string()
        
        } else if self.negative && b.negative {

            if &comparison::run(self, b.to_owned()) == "greater" {

                "less".to_string()

            } else {

                "greater".to_string()

            }

        } else if self.negative {

            "less".to_string()

        } else if b.negative {

            "greater".to_string()

        } else {
            
            comparison::run(self, b.to_owned())

        }

    }

}
