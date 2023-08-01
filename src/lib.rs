mod bit;
mod fraction;
mod integer;
mod matrix;

#[derive(Copy, Clone)]
pub enum Bit { One, Zero }

#[derive(Clone)]
pub struct Integer(Vec<Bit>);

#[derive(Clone)]
pub struct Fraction(Integer, Integer);

#[derive(Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
