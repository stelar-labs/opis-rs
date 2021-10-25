
use std::error::Error;
use std::fmt;

mod base10;

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

    pub fn add(mut self, mut b: Int) -> Self {

        let mut res = Int { bits: vec![] };

        let mut carry = 0;

        while self.bits.len() > 0 || b.bits.len() > 0 {

            let a_bit = match self.bits.pop() { Some(r) => r, None => 0 };

            let b_bit = match b.bits.pop() { Some(r) => r, None => 0 };

            let addition = carry + a_bit + b_bit;

            match addition {
                3 => { res.bits.push(1); carry = 1 },
                2 => { res.bits.push(0); carry = 1 },
                1 => { res.bits.push(1); carry = 0 },
                _ => { res.bits.push(0); carry = 0 }
            }

        }

        if carry != 0 { res.bits.push(1) }

        res.bits.reverse();

        res

    }

    pub fn sub(mut self, mut b: Int) -> Result<Self, Box<dyn Error>> {

        match &self.clone().cmp(&b)[..] {
            
            "greater" => {

                let mut res = Int { bits: vec![] };

                while self.bits.len() > 0 || b.bits.len() > 0 {

                    let a_bit = match self.bits.pop() { Some(r) => r, None => 0 };

                    let b_bit = match b.bits.pop() { Some(r) => r, None => 0 };

                    if b_bit > a_bit {

                        let mut borrowed: bool = false;
                        
                        let mut borrow_index = self.bits.len() - 1;

                        while !borrowed {

                            if self.bits[borrow_index] > 0 { self.bits[borrow_index] = 0; borrowed = true; }
                            
                            else { borrow_index -= 1 }

                        }

                        let diff = 2 - b_bit;

                        res.bits.push(diff);

                    } else {

                        let diff = a_bit - b_bit;

                        res.bits.push(diff);

                    }

                }

                res.bits.reverse();

                while res.bits[0] == 0 { res.bits.remove(0); };

                Ok(res)

            },

            _ => {
                Err(Box::new(CustomError("b is greater than a!".into())))
            }

        }

    }

    pub fn mul(self, b: &Int) -> Self {

        let mut res: Int = self.clone();

        b.bits
            .iter()
            .skip(1)
            .for_each(|&x| {

                res = res.clone().add(res.clone());
                
                if x == 1 { res = res.clone().add(self.clone()) }
            
            });

        res

    }

    // pub fn divide(self, d: &Int) -> Result<Self, Box<dyn Error>> {
    //     Ok(self)
    // }

    // pub fn modulo(self, n: &Int) -> Self {
    //     self
    // }

    // pub fn power(self, _p: &Int) -> Self {
    //     self
    // }

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

    // string conversion functions

    pub fn from_str(s: &str, r: u8) -> Result<Self, Box<dyn Error>> {

        match r {
            10 => {
                let b = base10::from(s)?;
                Ok(Self { bits: b })
            },
            _ => Err(Box::new(CustomError("base unsupported!".into())))
        }

    }

    pub fn to_str(self, r: u8) -> Result<String, Box<dyn Error>> {
        
        match r {
            
            2 => {

                let mut res: String = String::with_capacity(self.bits.len());
    
                for bit in self.bits { res.push_str(&bit.to_string()) }

                Ok(res)

            },
            _ => Err(Box::new(CustomError("base unsupported".into())))
        }

    }

    // comparison function

    pub fn cmp(self, b: &Int) -> String {

        let a_len = self.bits.len();

        let b_len = b.bits.len();

        if a_len > b_len {
            return "greater".to_string()

        } else if a_len < b_len { 
            return "lesser".to_string()
        
        } else {

            if self.bits[0] > b.bits[0] {
                return "greater".to_string()

            } else if self.bits[0] < b.bits[0] {
                return "lesser".to_string()
                
            } else {
                return "equal".to_string()

            }
        }

    }

}
