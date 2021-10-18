
use std::error::Error;

pub fn from(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {

    let mut binary_str: String = String::new();

    let mut integer_str: String = s.to_string();

    while integer_str != "0".to_string() {

        let (s_half, rem) = half(&integer_str);

        integer_str = s_half;

        binary_str.push_str(&rem.to_string());

    }

    let mut res: Vec<u8> = Vec::new();

    (0..binary_str.len())
        .step_by(8)
        .for_each(|x| {
            
            let end =
                if (x + 8) > binary_str.len() {
                    binary_str.len()
                } else { x + 8 };

            let slice: &str = &binary_str[x..end];

            let rev_slice: String = slice.chars().rev().collect::<String>();

            let byte: u8 = u8::from_str_radix(&rev_slice, 2).unwrap();

            res.push(byte);

        });

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