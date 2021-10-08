
use std::fmt::Write;

pub fn to_str(bytes: Vec<u8>) -> String {

    let mut s: String = String::with_capacity(bytes.len() * 8);

    for b in bytes {

        let mut b_str: String = String::new();

        write!(&mut b_str, "{:b}", b).unwrap();

        let rev_b_str: String = b_str.chars().rev().collect::<String>();

        s.push_str(&rev_b_str);

    }

    s

}