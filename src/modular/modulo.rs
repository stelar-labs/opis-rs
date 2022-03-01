use crate::Int;

pub fn main(a: &Int, b: &Int) -> Int {

    if a == &Int::zero() {
        Int::zero()
    }
    
    else if b == &Int::zero() {
        panic!("a/0 is undefined!")
    }
    
    else {

        let mut res = a % b;

        while res < Int::zero() {
            res += b.clone()
        }

        res

    }   

}