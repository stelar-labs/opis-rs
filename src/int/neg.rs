use crate::{Int, Bit};

impl Int {

    pub fn negative(&self) -> Int {

        if self.bits[0] == Bit::Zero {

            let mut bits = vec![Bit::One];

            let mut not_bits: Vec<Bit> =
                
                self.bits[1..]
                    .iter()
                    .map(|x| !x)
                    .collect();

            let start = not_bits.len() - 1;

            if not_bits[start] == Bit::Zero {

                not_bits[start] = Bit::One

            } else {

                not_bits[start] = Bit::Zero;

                let mut carry = Bit::One;

                if not_bits.len() > 1 {

                    for i in 1..not_bits.len() {

                        let bit_index = start - i;

                        let bit = not_bits[bit_index];

                        let (cy, sm) = carry + bit;

                        carry = cy;

                        not_bits[bit_index] = sm;

                    }

                }

            }

            bits = [bits, not_bits].concat();

            Int { bits } 

        } else {

            self.clone()
        
        }

    }

}