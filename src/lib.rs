
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

    // basic arithmetic

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

    pub fn sub(self, b: &Int) -> Result<Self, Box<dyn Error>> {

        match &self.cmp(b)[..] {
            
            "greater" => {

                let res = Int::from_str("10", 10)?;
            
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
