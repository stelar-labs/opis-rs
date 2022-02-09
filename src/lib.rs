mod bit;
mod int;

#[derive(Copy, Clone, Debug)]
pub enum Bit {One, Zero}

#[derive(Clone, Debug)]
pub struct Int { pub bits: Vec<Bit> }

pub fn mod_inv(a: &Int, m: &Int) -> Int {
    int::mod_inv::main(a, m)
}

pub fn modulo(a: &Int, b: &Int) -> Int {
    int::modulo::main(a, b)
}

pub fn pow(a: &Int, e:&Int) -> Int {
    int::pow::exponentiation(a, e)
}
