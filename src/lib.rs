mod bit;
mod int;
// mod modular;

#[derive(Copy, Clone, Debug)]
pub enum Bit {
    One,
    Zero
}

#[derive(Clone, Debug)]
pub struct Int {
    pub bits: Vec<Bit>
}
