use opis::Int;

#[test]
fn test_add() {
    assert_eq!(Int::one() + Int::one(), Int::two());
}

#[test]
fn test_sub() {
    assert_eq!(Int::one() - Int::one(), Int::zero());
}

#[test]
fn test_mul() {
    assert_eq!(Int::one() * Int::one(), Int::one());
}

#[test]
fn test_div() {
    assert_eq!((Int::one() / Int::one()).unwrap(), Int::one());
}

#[test]
fn test_rem() {
    assert_eq!((Int::one() % Int::one()).unwrap(), Int::zero());
}

#[test]
fn test_eq() {
    assert_eq!(Int::one() == Int::one(), true);
}

#[test]
fn test_not_eq() {
    assert_eq!(Int::one() == Int::zero(), false);
}

#[test]
fn test_gt() {
    assert_eq!(Int::one() > Int::zero(), true);
}

#[test]
fn test_lt() {
    assert_eq!(Int::zero() < Int::one(), true);
}
