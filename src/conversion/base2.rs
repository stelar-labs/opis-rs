use crate::Bit;

pub fn from(s: &str) -> Vec<Bit> {
    
    let mut split_str: Vec<_> = s.split("").collect();
    
    split_str.retain(|&x| x != "");

    let res: Vec<Bit> = split_str
        .iter()
        .skip(2)
        .map(|&x| {
            match x {
                "0" => Bit::Zero,
                "1" => Bit::One,
                _ => panic!("Unsupported bit type {}", x)
            }
        })
        .collect();

    res

}

pub fn to(bits: Vec<Bit>) -> String {

    let mut res: String = String::with_capacity(bits.len() + 2);

    res.push_str("b'");

    for bit in bits {
        match bit {
            Bit::Zero => res.push('0'),
            Bit::One => res.push('1')
        }
    };

    res

}