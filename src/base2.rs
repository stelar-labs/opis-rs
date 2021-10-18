
use std::fmt::Write;

pub fn to_str(bytes: Vec<u8>) -> String {

    let mut s: String = String::with_capacity(bytes.len() * 8);

    let mut s_vec: Vec<String> = Vec::new();

    for b in bytes {

        let mut b_str: String = String::new();

        write!(&mut b_str, "{:b}", b).unwrap();

        s_vec.push(b_str);

    }

    s_vec.reverse();

    for i in s_vec {

        s.push_str(&i)
        
    }

    s

}