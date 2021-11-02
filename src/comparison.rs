
use crate::Int;

pub fn run(mut a: Int, mut b: Int) -> String {

    let a_len = a.bits.len();

    let b_len = b.bits.len();

    if a_len > b_len {
        
        return "greater".to_string()

    } else if a_len < b_len { 
        
        return "less".to_string()
    
    } else {

        while a.bits[0] == b.bits[0] {
            
            a.bits.remove(0);
            
            b.bits.remove(0);
        
        }

        if a.bits[0] == 1 {

            return "greater".to_string();

        } else {
            
            return "less".to_string();
        
        }

    }

}