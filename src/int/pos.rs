use crate::{Int, Bit};

impl Int {

    pub fn positive(&self) -> Int {

        if self.bits[0] == Bit::One {

            let mut bits = self.bits[1..].to_vec();

            let last_index = bits.len() - 1;
            
            if bits[last_index] == Bit::Zero {

                let mut index = last_index;

                let mut borrowed = false;

                while !borrowed && index > 0 {

                    let bit = bits[index];

                    if bit == Bit::Zero {

                        index -= 1;

                    } else {

                        bits[index] = Bit::Zero;

                        borrowed = true;

                    }
                    
                }

            } else {
                
                bits[last_index] = Bit::Zero

            }

            bits =

                bits
                    .iter()
                    .map(|x| !x)
                    .collect();

            bits = [vec![Bit::One], bits].concat();

            Int { bits }

        } else {

            self.clone()

        }

    }

}