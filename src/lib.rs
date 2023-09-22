use std::any::Any;

mod bit;
mod fraction;
mod integer;
mod matrix;

#[derive(Copy, Clone, Hash)]
pub enum Bit {
    One,
    Zero
}

#[derive(Clone, Hash)]
pub struct Integer(pub Vec<Bit>);

#[derive(Clone, Hash)]
pub struct Fraction(
    pub Integer,
    pub Integer
);

#[derive(Clone, Hash)]
pub struct Irrational(
    pub String,
    pub Option<Real>
);

#[derive(Clone, Hash)]
pub enum Real {
    Integer,
    Fraction,
    Irrational
}

#[derive(Clone, Hash)]
pub struct Complex(
    pub Real,
    pub Real
);

#[derive(Clone)]
pub struct Matrix<'a>(pub Vec<Vec<MatrixElement<'a>>>);

#[derive(Clone, Hash)]
pub enum MatrixElement<'a> {
    Borrowed(&'a dyn Any),
    Owned(dyn Any)
}
