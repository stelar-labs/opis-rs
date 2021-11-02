
use crate::adder;

pub fn run(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {

    let mut res: Vec<u8> = a.to_owned();

    b
    .iter()
    .skip(1)
    .for_each(|&x| {

        res = adder::run(res.to_owned(), res.to_owned());
        
        if x == 1 {

            res = adder::run(res.to_owned(), a.to_owned());
        
        }
    
    });

    res

}