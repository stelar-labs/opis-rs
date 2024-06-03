// mod bit;
// mod fraction;
// mod integer;
// mod matrix;

// #[derive(Copy, Clone, Hash)]
// pub enum Bit { One, Zero }

// #[derive(Clone, Hash)]
// pub struct Integer(pub Vec<Bit>);

// #[derive(Clone, Hash)]
// pub struct Fraction(pub Integer, pub Integer);

// #[derive(Clone)]
// pub struct Matrix<T>(pub Vec<Vec<T>>);

mod integer;
pub use integer::Integer;