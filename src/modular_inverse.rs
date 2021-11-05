
use crate::Int;

pub fn run(mut a: Int, m: &Int) -> Int {

    let mut modulus: Int = m.to_owned();

    let mut t1: Int = Int::one();

    let mut t2: Int = Int::zero();
    
    a = a.modulo(m).unwrap();

    while a.to_owned().is_greater(&Int::one()) {

        let ratio = modulus.to_owned().div(&a).unwrap();

        let t1_x_ratio = t1.to_owned().mul(&ratio);

        let t1_new = t2.to_owned().sub(&t1_x_ratio);

        t2 = t1.to_owned();

        t1 = t1_new;

        let a_x_ratio = a.to_owned().mul(&ratio);

        let a_new = modulus.sub(&a_x_ratio);

        modulus = a;

        a = a_new;

    }

    t1.to_owned().rem(m).unwrap()

}