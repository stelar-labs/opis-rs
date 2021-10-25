
use std::error::Error;

pub fn from(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {

    let mut res: Vec<u8> = vec![];

    let mut i_str = s.to_string();

    while i_str != "0".to_string() {

        let (s_half, rem) = half(&i_str);

        i_str = s_half;

        res.push(rem);

    }

    res.reverse();

    Ok(res)

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
            
                if d != 0 { res.push_str(&d.to_string()) }

            }

        } else {

            res.push_str(&d.to_string())

        }
        
        rem = n%2;

    }

    (res, rem)

}