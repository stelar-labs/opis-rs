mod bit;
mod int;
mod modular;

#[derive(Copy, Clone, Debug)]
pub enum Bit {
    One,
    Zero
}

#[derive(Clone, Debug)]
pub struct Int {
    pub magnitude: Vec<Bit>,
    pub sign: bool
}

pub fn modulo(a: &Int, b: &Int) -> Int {
    modular::modulo::main(a, b)
}

pub fn pow(a: &Int, e:&Int) -> Int {
    int::pow::exponentiation(a, e)
}
