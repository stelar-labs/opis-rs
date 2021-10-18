
use std::error::Error;
use std::fmt;

mod base2;
mod base10;

#[derive(Clone, Debug)]
pub struct Int { pub bytes: Vec<u8> }

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opis Error: {}", self.0)
    }
}

impl Error for CustomError {}

impl Int {

    // arithmetic

    pub fn add(self, b: &Int) -> Self {

        let a_len = self.bytes.len(); let b_len = b.bytes.len();

        let res_len = if a_len > b_len { a_len } else { b_len };

        let mut carry = 0; let mut res = Int { bytes: vec![] };

        for x in 0..res_len {

            let a_byte = if x < a_len { self.bytes[x] } else { 0 };

            let b_byte = if x < b_len { b.bytes[x] } else { 0 };

            let s: u16 = a_byte as u16 + carry as u16 + b_byte as u16;

            if s > 255 {
                
                res.bytes.push((s - 256) as u8); carry = 1;

            } else {

                res.bytes.push(s as u8); carry = 0;

            }

        }

        if carry == 1 { res.bytes.push(carry) }

        res

    }

    pub fn sub(mut self, b: &Int) -> Result<Self, Box<dyn Error>> {

        match &self.clone().cmp(b)[..] {
            
            "greater" => {

                let mut res = Int { bytes: vec![] };

                for x in 0..self.bytes.len() {

                    let a_byte = self.bytes[x] as u16;

                    let b_byte = b.bytes[x] as u16;

                    if b_byte > a_byte {

                        let mut borrowed: bool = false;

                        let mut borrow_index = x + 1;

                        while !borrowed {

                            if self.bytes[borrow_index] > 0 {

                                self.bytes[borrow_index] -= 1;

                                borrowed = true;

                            } else {

                                borrow_index += 1;

                            }
                        }
                        
                        let diff = ((256 + a_byte) - b_byte) as u8;

                        res.bytes.push(diff);

                    } else {

                        let diff = (a_byte - b_byte) as u8;

                        res.bytes.push(diff);

                    }

                }

                while res.bytes[res.bytes.len() - 1] == 0 {

                    res.bytes.remove(res.bytes.len() - 1);
                    
                }
            
                Ok(res)

            },

            _ => {
                Err(Box::new(CustomError("b is greater than a!".into())))
            }

        }

    }

    pub fn mul() {}

    pub fn div() {}

    pub fn rem() {}

    pub fn pow(self, _p: &Int) -> Self {
        self
    }

    // string conversion functions

    pub fn from_str(s: &str, r: u8) -> Result<Self, Box<dyn Error>> {

        match r {
            10 => {
                let b = base10::from(s)?;
                Ok(Self { bytes: b })
            },
            _ => Err(Box::new(CustomError("base unsupported!".into())))
        }

    }

    pub fn to_str(self, r: u8) -> Result<String, Box<dyn Error>> {
        
        match r {
            2 => {
                let s = base2::to_str(self.bytes);
                Ok(s)
            },
            _ => Err(Box::new(CustomError("base unsupported".into())))
        }

    }

    // comparison function

    pub fn cmp(self, b: &Int) -> String {

        let a_len = self.bytes.len();

        let b_len = b.bytes.len();

        if a_len > b_len {
            return "greater".to_string()

        } else if a_len < b_len { 
            return "lesser".to_string()
        
        } else {

            if self.bytes[a_len - 1] > b.bytes[b_len - 1] {
                return "greater".to_string()

            } else if self.bytes[a_len - 1] < b.bytes[b_len - 1] {
                return "lesser".to_string()
                
            } else {
                return "equal".to_string()

            }
        }

    }

}
