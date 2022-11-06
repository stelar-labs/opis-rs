use crate::{Integer, Bit};

pub fn run(numerator: &Integer, denominator: &Integer) -> (Integer, Integer) {

    let mut quotient = Integer::zero();

    let mut remainder = Integer(numerator.0[..1].to_vec());

    numerator.0
        .iter()
        .skip(1)
        .for_each(|x| {

            remainder.0.push(*x);

            if &remainder > denominator {

                quotient.0.push(Bit::One);
                
                remainder = &remainder - denominator;
                
            } else if &remainder == denominator {

                quotient.0.push(Bit::One);

                remainder = Integer::zero();

            } else {

                quotient.0.push(Bit::Zero);

            };

        });
    
    quotient.clean();

    remainder.clean();

    (quotient, remainder)

}
