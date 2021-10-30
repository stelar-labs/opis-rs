
use std::error::Error;

use crate::Int;
use crate::CustomError;

pub fn run(mut a: Int, mut b: Int) -> Result<Int, Box<dyn Error>> {

    match &a.clone().cmp(&b)[..] {
            
        "less" => Err(Box::new(CustomError("b is greater than a!".into()))),

        _ => {

            let mut res = Int {
                bits: vec![]
            };

            while a.bits.len() > 0 || b.bits.len() > 0 {

                let a_bit = match a.bits.pop() {
                    Some(r) => r, 
                    None => 0
                };

                let b_bit = match b.bits.pop() {
                    Some(r) => r,
                    None => 0
                };

                if b_bit > a_bit {

                    let mut borrowed: bool = false;
                    
                    let mut borrow_index = a.bits.len() - 1;

                    while !borrowed {

                        if a.bits[borrow_index] == 1 { 
                            
                            a.bits[borrow_index] = 0;
                            borrowed = true;
                        
                        } else {

                            a.bits[borrow_index] = 1;
                            borrow_index -= 1;
                        
                        }

                    }
                    
                    res.bits.push(1);

                } else {

                    let diff = a_bit - b_bit;
                    
                    res.bits.push(diff);
                
                }

            }

            res.bits.reverse();

            while res.bits.len() > 1 && res.bits[0] == 0 {
                
                res.bits.remove(0);

            }

            Ok(res)

        }

    }

}