use crate::Int;
use crate::operators;
use std::cmp::Ordering;

pub fn main(a: Int, b: Int) -> Int {

    match (a.sign, b.sign) {

        (false, false) => {

            match operators::comparator::main(&a.bits, &b.bits) {

                Ordering::Equal => Int::zero(),

                Ordering::Greater => Int {
                    bits: operators::subtractor::main(&a.bits, &b.bits),
                    sign: false
                },

                Ordering::Less => Int {
                    bits: operators::subtractor::main(&b.bits, &a.bits),
                    sign: true
                }

            }

        },

        (true, true) => {

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

        (true, false) => Int {
            bits: operators::adder::main(&a.bits, &b.bits),
            sign: true
        },

        (false, true) => Int {
            bits: operators::adder::main(&a.bits, &b.bits),
            sign: false
        }

    }

}