
use crate::Bit;

pub fn from(arg: &str) -> Vec<Bit> {

    let mut res: Vec<Bit> = Vec::new();

    let mut i_str = arg.to_string();

    while i_str != "0".to_string() {

        let (s_half, rem) = half(&i_str);

        i_str = s_half;

        match rem {
            0 => res.push(Bit::Zero),
            1 => res.push(Bit::One),
            _ => panic!("Unsupported bit type {}", rem)
        }

    }

    res.reverse();

    if res.is_empty() {
        res = vec![Bit::Zero]
    }
    
    res

}

pub fn to(bits: &Vec<Bit>) -> String {

    let mut res: String = "0".to_string();

    for b in bits {

        res = double(&res);

        if b == &Bit::One {
            res = add_one(&res);
        }

    }
    
    res

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