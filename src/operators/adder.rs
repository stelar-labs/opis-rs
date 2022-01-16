use crate::Bit;

pub fn main(a: &Vec<Bit>, b: &Vec<Bit>) -> Vec<Bit> {
    
    let mut a_bits = a.clone();
    
    let mut b_bits = b.clone();

    let mut res: Vec<Bit> = Vec::new();
    
    let mut carry: Bit = Bit::Zero;

    while !a_bits.is_empty() || !b_bits.is_empty() {

        let a_bit: Bit =
            match a_bits.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        let b_bit: Bit =
            match b_bits.pop() {
                Some(r) => r,
                None => Bit::Zero
            };

        match (carry, a_bit, b_bit) {
            
            (Bit::One, Bit::One, Bit::One) => { res.push(Bit::One); carry = Bit::One },
            
            (Bit::Zero, Bit::One, Bit::One) => { res.push(Bit::Zero); carry = Bit::One },
            
            (Bit::One, Bit::Zero, Bit::One) => { res.push(Bit::Zero); carry = Bit::One },
            
            (Bit::One, Bit::One, Bit::Zero) => { res.push(Bit::Zero); carry = Bit::One },
            
            (Bit::One, Bit::Zero, Bit::Zero) => { res.push(Bit::One); carry = Bit::Zero },
            
            (Bit::Zero, Bit::One, Bit::Zero) => { res.push(Bit::One); carry = Bit::Zero },
            
            (Bit::Zero, Bit::Zero, Bit::One) => { res.push(Bit::One); carry = Bit::Zero },
            
            (Bit::Zero, Bit::Zero, Bit::Zero) => { res.push(Bit::Zero); carry = Bit::Zero }

        }

    }

    if carry == Bit::One {
        res.push(Bit::One);
    }

    res.reverse();

    res

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_add_one() {
        assert_eq!(
            main(&vec![Bit::One], &vec![Bit::One]),
            vec![Bit::One, Bit::Zero]
        )
    }
    #[test]
    fn three_add_one() {
        assert_eq!(
            main(&vec![Bit::One, Bit::One], &vec![Bit::One]),
            vec![Bit::One, Bit::Zero, Bit::Zero]
        )
    }
    #[test]
    fn one_add_three() {
        assert_eq!(
            main(&vec![Bit::One], &vec![Bit::One, Bit::One]),
            vec![Bit::One, Bit::Zero, Bit::Zero]
        )
    }
}