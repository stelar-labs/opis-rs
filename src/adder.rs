
pub fn run(mut a: Vec<u8>, mut b: Vec<u8>) -> Vec<u8> {

    let mut res: Vec<u8> = Vec::new();

    let mut carry = 0;

    while a.len() > 0 || b.len() > 0 {

        let a_bit = match a.pop() {
            Some(r) => r,
            None => 0
        };

        let b_bit = match b.pop() {
            Some(r) => r,
            None => 0
        };

        let addition = carry + a_bit + b_bit;

        match addition {
            3 => { res.push(1); carry = 1 },
            2 => { res.push(0); carry = 1 },
            1 => { res.push(1); carry = 0 },
            _ => { res.push(0); carry = 0 }
        }

    }

    if carry != 0 {
        res.push(1)
    }

    res.reverse();

    res

}
