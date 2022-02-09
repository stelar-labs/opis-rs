use crate::Bit;

pub fn from(s: &str) -> Vec<Bit> {
    
    let mut res: Vec<Bit> = Vec::new();
    
    let mut hex_str_vec: Vec<_> = s.split("").collect();
    
    hex_str_vec.retain(|&x| x != "");

    hex_str_vec
        .iter()
        .for_each(|x| {
            
            let byte = u8::from_str_radix(&format!("{}", x), 16).unwrap();

            let binary_str = format!("{:04b}", byte);

            let mut binary_str_vec: Vec<_> = binary_str.split("").collect();
    
            binary_str_vec.retain(|&x| x != "");

            binary_str_vec
                .iter()
                .for_each(|&y| {
                    
                    match y {
                        "0" => res.push(Bit::Zero),
                        "1" => res.push(Bit::One),
                        _ => panic!("Unsupported bit type {}", x)
                    }

                });

        });

    while res.len() > 1 && res[0] == Bit::Zero {
        res.remove(0);
    }

    res

}

pub fn to(bytes: Vec<u8>) -> String {
    
    let mut res = String::from("0x");

    bytes
        .iter()
        .for_each(|x| {

            let hex_str = format!("{:02X}", x);

            res.push_str(&hex_str)

        });
        
    res

}