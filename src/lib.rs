
use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct Int { bytes: Vec<u8> }

impl Int {

    pub fn add(self, n: &Int) -> Self {

        let a_len = self.bytes.len();
        
        let b_len = n.bytes.len();

        let res_len =
            if a_len > b_len { a_len }
            else { b_len };

        let mut carry = 0;

        let mut res = Int { bytes: vec![] };

        for x in 0..res_len {

            let a_byte =
                if x < a_len { self.bytes[x] }
                else { 0 };

            let b_byte = 
                if x < b_len { n.bytes[x] }
                else { 0 };

            let s: u16 = a_byte as u16 + carry as u16 + b_byte as u16;

            if s > 255 {
                
                res.bytes.push((s - 256) as u8); carry = 1;

            } else {

                res.bytes.push(s as u8); carry = 0;

            }

        }

        if carry == 1 {

            res.bytes.push(carry);

        }

        res

    }

    pub fn from(s: &str) -> Self {

        let mut b_str: String = String::new();

        let mut int_str: String = s.to_string();

        while int_str > "0".to_string() {
        
            let (s_half, rem) = half(&int_str);
    
            int_str = s_half;
    
            b_str.push_str(&rem.to_string());
    
        }

        let mut res: Vec<u8> = Vec::new();

        (0..b_str.len())
            .step_by(8)
            .for_each(|x| {
                
                let end =
                    if x + 8 > b_str.len() { b_str.len() }
                    else { x + 8 };

                let slice: &str = &b_str[x..end];

                let rev_slice: String = slice.chars().rev().collect::<String>();

                let byte: u8 = u8::from_str_radix(&rev_slice, 2).unwrap();

                res.push(byte);

            });

        Self { bytes: res }

    }

    pub fn as_binary(self) -> String {
        
        let mut s: String = String::with_capacity(self.bytes.len() * 8);

        for b in self.bytes {

            let mut b_str: String = String::new();

            write!(&mut b_str, "{:b}", b).unwrap();

            let rev_b_str: String = b_str.chars().rev().collect::<String>();

            s.push_str(&rev_b_str);

        }

        s
        
    }

}

fn half(s: &str) -> (String, u8) {

    let mut split: Vec<_> = s.split("").collect();
    
    split.retain(|&x| x != "");

    let mut res: String = String::new();

    let mut rem: u8 = 0;

    for i in split {

        let n_str = format!("{}{}", rem, i);

        let n = u8::from_str_radix(&n_str, 10).unwrap();

        let d = n/2;

        if res == String::new() {

            if d != 0 {
                res.push_str(&d.to_string());
            }

        } else {

            res.push_str(&d.to_string());

        }
        
        rem = n%2;

    }

    (res, rem)

}
