
use std::error::Error;
use std::fmt;

mod base10;

mod addition;
mod subtraction;
mod multiplication;
mod division;
mod exponentiation;
mod comparison;

#[derive(Clone, Debug)]
pub struct Int { pub bits: Vec<u8> }

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opis Error: {}", self.0)
    }
}

impl Error for CustomError {}

impl Int {

    pub fn add(self, b: &Int) -> Self {

        addition::run(self, b.to_owned())

    }

    pub fn sub(self, b: &Int) -> Result<Self, Box<dyn Error>> {

        subtraction::run(self, b.to_owned())

    }

    pub fn mul(self, b: &Int) -> Self {

        multiplication::run(self, b)

    }

    pub fn div(self, b: &Int) -> Result<Self, Box<dyn Error>> {

        let zero = Int {
            bits: vec![0]
        };

        if self.bits == zero.bits {

            Ok(zero)

        } else if b.bits == vec![0] {

            Err(Box::new(CustomError("a/0 is undefined! ".into())))
    
        } else {

            let (q, _) = division::run(self, b);
            
            Ok(q)

        }

    }

    pub fn rem(self, b: &Int) -> Result<Self, Box<dyn Error>> {

        let zero = Int {
            bits: vec![0]
        };

        if self.bits == zero.bits {

            Ok(zero)

        } else if b.bits == vec![0] {

            Err(Box::new(CustomError("a/0 is undefined! ".into())))
    
        } else {

            let (_, r) = division::run(self, b);

            Ok(r)

        }

        

    }

    pub fn pow(self, b: &Int) -> Self {

        exponentiation::run(self, b)

    }

    // pub fn modular_inverse(self, m: &Int) -> Self {

    //     let modulus = m;
        
    //     let t1 = Int::from_str("1", 10).unwrap();
    //     let t2 = Int::from_str("0", 10).unwrap();

    //     let residue = self.modulo(&m);

    //     let high = m;

    //     while &residue.clone().cmp(&Int::from_str("1", 10).unwrap()) == "greater" {

    //         let ratio = high.clone().divide(&residue).unwrap();
    //     }

    //     self
    // }

    pub fn from_str(s: &str, r: u8) -> Result<Self, Box<dyn Error>> {

        match r {
            
            10 => {

                let b = base10::from(s)?;

                let res = Int {
                    bits: b
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
    
                for bit in self.bits { res.push_str(&bit.to_string()); };

                Ok(res)

            },
            _ => Err(Box::new(CustomError("unsupported radix!".into())))
        }

    }

    pub fn cmp(self, b: &Int) -> String {

        comparison::run(self, b.to_owned())

    }

}
