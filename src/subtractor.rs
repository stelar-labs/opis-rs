
pub fn run(mut a: Vec<u8>, mut b: Vec<u8>) -> Vec<u8> {
    
    let mut res: Vec<u8> = Vec::new();

    while a.len() > 0 || b.len() > 0 {

        let a_bit = match a.pop() {
            Some(r) => r, 
            None => 0
        };

        let b_bit = match b.pop() {
            Some(r) => r,
            None => 0
        };

        if b_bit > a_bit {

            let mut borrowed: bool = false;
            
            let mut borrow_index = a.len() - 1;

            while !borrowed {

                if a[borrow_index] == 1 { 
                    
                    a[borrow_index] = 0;
                    borrowed = true;
                
                } else {

                    a[borrow_index] = 1;
                    borrow_index -= 1;
                
                }

            }
            
            res.push(1);

        } else {

            let diff = a_bit - b_bit;
            
            res.push(diff);
        
        }

    }

    res.reverse();

    while res.len() > 1 && res[0] == 0 {
        
        res.remove(0);

    }

    res

}