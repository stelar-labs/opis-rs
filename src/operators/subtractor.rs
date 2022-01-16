use crate::Bit;

pub fn main(a: &Vec<Bit>, b: &Vec<Bit>) -> Vec<Bit> {

    let mut a_bits = a.clone();
    
    let mut b_bits = b.clone();
    
    let mut res: Vec<Bit> = Vec::new();

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
        
        match (a_bit, b_bit) {
            
            (Bit::Zero, Bit::Zero) => res.push(Bit::Zero),
            
            (Bit::One, Bit::One) => res.push(Bit::Zero),
            
            (Bit::One, Bit::Zero) => res.push(Bit::One),
            
            (Bit::Zero, Bit::One) => {

                let mut borrowed: bool = false;

                let mut i: usize = a_bits.len() - 1;

                while !borrowed {

                    if a_bits[i] == Bit::One {
                        a_bits[i] = Bit::Zero;
                        borrowed = true;
                    }
                    
                    else {
                        a_bits[i] = Bit::One;
                        i -= 1;
                    }
    
                }
                
                res.push(Bit::One);

            }
        }

    }

    res.reverse();

    while res.len() > 1 && res[0] == Bit::Zero {
        res.remove(0);
    }

    res

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_sub_one() {
        assert_eq!(
            main(&vec![Bit::One], &vec![Bit::One]),
            vec![Bit::Zero]
        )
    }
    #[test]
    fn three_sub_three() {
        assert_eq!(
            main(&vec![Bit::One, Bit::One], &vec![Bit::One, Bit::One]),
            vec![Bit::Zero]
        )
    }
    #[test]
    fn three_sub_one() {
        assert_eq!(
            main(&vec![Bit::One, Bit::One], &vec![Bit::One]),
            vec![Bit::One, Bit::Zero])
    }
    #[test]
    fn five_sub_two() {
        assert_eq!(
            main(&vec![Bit::One, Bit::Zero, Bit::One], &vec![Bit::One, Bit::Zero]),
            vec![Bit::One, Bit::One]
        )
    }
}