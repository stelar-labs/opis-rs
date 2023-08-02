mod bit;
mod fraction;
mod integer;
mod matrix;

#[derive(Copy, Clone)]
pub enum Bit { One, Zero }

#[derive(Clone)]
pub struct Integer(pub Vec<Bit>);

#[derive(Clone)]
pub struct Fraction(pub Integer, pub Integer);

#[derive(Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
