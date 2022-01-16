
use opis::{Int, pow, mod_inv};

#[test]
fn addition() {
    let a = Int::from_str("3", 10);
    let b = Int::from_str("1", 10);
    assert_eq!((&a + &b).to_str(10), "4");
}

// fn main() {

    // let a = Int::from_str("3", 10);

    // let b = Int::from_str("11", 10);

    // assert_eq!((&a + &b).to_str(10), "2059");

    // assert_eq!((&b - &a).to_str(10), "23");

    // assert_eq!((&a * &b).to_str(10), "78");

    // assert_eq!((&b / &a).to_str(10), "8");

    // assert_eq!((&b % &a).to_str(10), "2");

    // assert_eq!(pow(&b, &a).to_str(10), "17576");

    // assert_eq!(mod_inv(&a, &b).to_str(10), "4");

// }