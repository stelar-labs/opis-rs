mod bit;
mod int;

#[derive(Copy, Clone, Debug)]
pub enum Bit { One, Zero }

#[derive(Clone)]
pub struct Int { bits: Vec<Bit> }
