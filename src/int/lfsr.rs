use crate::Int;
use std::error::Error;

impl Int {

    pub fn lfsr(&self, iterations: usize) -> Result<Self, Box<dyn Error>> {

        let mut result = self.clone();
        
        let mut f: Vec<usize> = vec![1,2];

        let mut next_f = f[f.len() - 2] + f[f.len() - 1];

        while next_f < self.bits.len() {

            f.push(next_f);

            next_f = f[f.len() - 2] + f[f.len() - 1];
            
        }

        for _ in 0..iterations {

            let output = 

                f
                .iter()
                .skip(1)
                .fold(
                    result.bits[result.bits.len() - 1],
                    |acc, x|
                    {

                        let bit = result.bits[result.bits.len() - x];

                        acc ^ bit
                    
                    }
                );

            result.bits.pop();

            result.bits = [vec![output], result.bits].concat();

        }

        Ok(result)

    }

}
