#[cfg(target_pointer_width = "64")]
pub type Digit = u64;

#[cfg(not(target_pointer_width = "64"))]
pub  type Digit = u32;

/// Returns the least significant bit (LSB) of a digit as a boolean.
pub fn get_least_significant_bit(digit: Digit) -> bool {
    (digit & 1) != 0
}

/// Returns the most significant bit (MSB) of a digit as a boolean.
pub fn get_most_significant_bit(digit: Digit) -> bool {
    if cfg!(target_pointer_width = "64") {
        ((digit >> 63) & 1) != 0
    } else {
        ((digit >> 31) & 1) != 0
    }
}