use crate::Bit;

pub fn from(s: &str) -> Vec<Bit> {
    
    let mut split_str: Vec<_> = s.split("").collect();
    
    split_str.retain(|&x| x != "");

    if split_str.len() > 3 {

        let res: Vec<Bit> = split_str
            .iter()
            .map(|&x| {
                match x {
                    "0" => Bit::Zero,
                    "1" => Bit::One,
                    _ => panic!("{} is not a supported bit type!", x)
                }
            })
            .collect();
        
        res

    } else {
        panic!("Too short!")
    }

}

pub fn to(bits: &Vec<Bit>) -> String {

    let mut res: String = String::new();

    res.push_str("b'");

    for bit in bits {
        match bit {
            Bit::Zero => res.push('0'),
            Bit::One => res.push('1')
        }
    };

    res

}