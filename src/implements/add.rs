use crate::Int;
use crate::operators::adder::main as adder;
use crate::operators;
use std::cmp::Ordering;

pub fn main(a: Int, b: Int) -> Int {

    match (a.sign, b.sign) {

        (false, false) => Int {
            bits: adder(&a.bits, &b.bits),
            sign: false
        },
        
        (true, true) => Int {
            bits: adder(&a.bits, &b.bits),
            sign: true
        },
        
        (true, false) => {
            
            match operators::comparator::main(&a.bits, &b.bits) {
                
                Ordering::Equal => Int::zero(),
                
                Ordering::Greater => Int {
                    bits: operators::subtractor::main(&a.bits, &b.bits),
                    sign: true
                },
                
                Ordering::Less => Int {
                    bits: operators::subtractor::main(&b.bits, &a.bits),
                    sign: false
                }

            }

        },

        (false, true) => {
            
            match operators::comparator::main(&a.bits, &b.bits) {
                
                Ordering::Equal => Int::zero(),
                
                Ordering::Less => Int {
                    bits: operators::subtractor::main(&a.bits, &b.bits),
                    sign: true
                },
                
                Ordering::Greater => Int {
                    bits: operators::subtractor::main(&b.bits, &a.bits),
                    sign: false
                }

            }

        }

    }

}
