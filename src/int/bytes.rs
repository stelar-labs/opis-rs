use crate::Bit;

pub fn from(bytes: &Vec<u8>) -> Vec<Bit> {

    let mut res: Vec<Bit> = Vec::new();

    let bin_str: String = bytes
        .iter()
        .fold(String::new(), |acc, x| format!("{}{:b}", acc, x));

    for i in bin_str.chars() {
        match i {
            '0' => res.push(Bit::Zero),
            '1' => res.push(Bit::One),
            _ => ()
        }
    }

    while res.len() > 1 && res[0] == Bit::Zero {
        res.remove(0);
    }

    res

}

pub fn to(mut bits: Vec<Bit>) -> Vec<u8> {

    let mut res: Vec<u8> = Vec::new();

    while !bits.is_empty() {

        let mut byte_str: String = String::new();

        while byte_str.len() < 8 {
            match bits.pop() {
                Some(r) => {
                    match r {
                        Bit::Zero => byte_str = format!("0{}", byte_str),
                        Bit::One => byte_str = format!("1{}", byte_str)
                    }
                },
                None => break
            }
        }

        res.push(u8::from_str_radix(&byte_str, 2).unwrap())

    }

    res.reverse();

    res

}