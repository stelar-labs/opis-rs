use crate::Int;

pub fn main(a: &Int, b: &Int) -> Int {

    if a == &Int::zero() {
        Int::zero()
    }
    
    else if b == &Int::zero() {
        panic!("a/0 is undefined!")
    }
    
    else {

        let r = a % b;

        if r.sign {
            &r + b
        }
        
        else {
            r
        }

    }   

} 