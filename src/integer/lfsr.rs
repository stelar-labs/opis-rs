use crate::Integer;

impl Integer {
    
    pub fn lfsr(&self, iterations: usize) -> Self {

        let mut result = self.clone();
        
        let mut fibs: Vec<usize> = vec![1,2];

        let mut next_fib = fibs[fibs.len() - 2] + fibs[fibs.len() - 1];

        while next_fib < self.0.len() {

            fibs.push(next_fib);

            next_fib = fibs[fibs.len() - 2] + fibs[fibs.len() - 1];
            
        }

        for _ in 0..iterations {

            let output = fibs
                .iter()
                .skip(1)
                .fold(
                    result.0[result.0.len() - 1],
                    |acc, x| acc ^ result.0[result.0.len() - x]
                );

            result.0.pop();

            result.0 = [vec![output], result.0].concat();

        }

        result

    }

}