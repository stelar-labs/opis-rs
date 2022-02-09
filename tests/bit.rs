
use opis::Bit;

#[test]
fn not_zero_is_one() {
    let zero_bit: Bit = Bit::Zero;
    assert_eq!(Bit::One, !zero_bit);
}