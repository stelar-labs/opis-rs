
use crate::Bit;

pub fn from(mut s: &str) -> Vec<Bit> {

    let mut sign_bit: Bit = Bit::Zero;

    let (first, last) = s.split_at(1);
    
    if first == "-" {
        sign_bit = Bit::One;
        s = last
    }

    let mut bits: Vec<Bit> = Vec::new();

    let mut i_str = s.to_string();

    while i_str != "0".to_string() {

        let (s_half, rem) = half(&i_str);

        i_str = s_half;

        match rem {
            0 => bits.push(Bit::Zero),
            1 => bits.push(Bit::One),
            _ => panic!("Unsupported bit type {}", rem)
        }

    }

    bits.reverse();

    if bits.is_empty() {
        bits = vec![Bit::Zero]
    }
    
    [vec![sign_bit], bits].concat()

}

pub fn to(bits: Vec<Bit>) -> String {

    let mut res: String = "0".to_string();

    for b in bits[1..].to_vec() {

        res = double(&res);

        if b == Bit::One {
            res = add_one(&res);
        }

    }

    if bits[0] == Bit::One {
        format!("-{}", res)
    }

    else {
        res
    }

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

#[cfg(test)]
mod tests {
    
    use super::*;

    use crate::Int;

    #[test]
    fn from_1() {
        assert_eq!(
            Int::from("1").bits,
            vec![Bit::Zero, Bit::One]
        )
    }

    #[test]
    fn from_991() {
        assert_eq!(
            Int::from("991").bits,
            vec![Bit::Zero, Bit::One, Bit::One, Bit::One, Bit::One, Bit::Zero, Bit::One, Bit::One, Bit::One, Bit::One, Bit::One]
        )
    }

    #[test]
    fn from_neg_991() {
        assert_eq!(
            Int::from("-991").bits,
            vec![Bit::One, Bit::One, Bit::One, Bit::One, Bit::One, Bit::Zero, Bit::One, Bit::One, Bit::One, Bit::One, Bit::One]
        )
    }

    #[test]
    fn to_991() {
        assert_eq!(
            Int {bits: vec![Bit::Zero, Bit::One, Bit::One, Bit::One, Bit::One, Bit::Zero, Bit::One, Bit::One, Bit::One, Bit::One, Bit::One]}.to(10),
            "991".to_string()
        )
    }

}