
use std::error::Error;
use std::fmt::Write;

pub fn from(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {

    let mut res: Vec<u8> = vec![];

    let mut hex_str_vec: Vec<_> = s.split("").collect();
    
    hex_str_vec.retain(|&x| x != "");

    (2..hex_str_vec.len())
        .for_each(|x| {
            
            let byte = u8::from_str_radix(&format!("{}", hex_str_vec[x]), 16).unwrap();

            let mut binary_str = format!("0000{:b}", byte);

            while binary_str.chars().count() > 4 {

                binary_str.remove(0);
    
            }

            let mut binary_str_vec: Vec<_> = binary_str.split("").collect();
    
            binary_str_vec.retain(|&x| x != "");

            for i in binary_str_vec {
                
                res.push(u8::from_str_radix(&i, 2).unwrap())

            }


        });

    Ok(res)

}

pub fn to(b: Vec<u8>) -> String {

    let mut res = String::from("0x");

    (0..b.len())
        .step_by(8)
        .for_each(|i| {

            let byte_str: String = b[i..i + 8].iter().fold(String::new(), |acc, x| format!("{}{}", acc, x));
            
            let byte: u8 = u8::from_str_radix(&byte_str, 2).unwrap();

            write!(&mut res, "{:X}", byte).unwrap()

        });

    res

}
