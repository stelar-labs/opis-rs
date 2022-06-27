use opis::Int;

#[test]
fn test_from_u8() {
    assert_eq!(Int::from_u8(&1).unwrap(), Int::one());
}
