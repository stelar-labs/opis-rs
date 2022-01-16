use crate::Int;

pub fn main(a: &Int, m: &Int) -> Int {

    let (g, mut x) = gcd(a.clone(), m.clone(), Int::zero(), Int::one()); 
    
    if g != Int::one() {
        panic!("Inverse doesn't exist!")
    } else {

        while x < Int::zero() {
            x += m.clone()
        }
        
        x
        
    }

}

fn gcd(a: Int, m: Int, x: Int, y: Int) -> (Int, Int) {

    if a == Int::zero() {
        (m, x)
    } else {

        let newa = &m % &a;
    
        let newy = x - &(&m / &a) * &y;
    
        gcd(newa, a, y, newy)
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_42_56() {
        assert_eq!(gcd(Int::from_str("42", 10), Int::from_str("56", 10), Int::zero(), Int::one()).0.to_str(10), "14")
    }
    #[test]
    fn gcd_461952_116298() {
        assert_eq!(gcd(Int::from_str("461952", 10), Int::from_str("116298", 10), Int::zero(), Int::one()).0.to_str(10), "18")
    }
    #[test]
    fn gcd_7966496_314080416() {
        assert_eq!(gcd(Int::from_str("7966496", 10), Int::from_str("314080416", 10), Int::zero(), Int::one()).0.to_str(10), "32")
    }
    #[test]
    fn gcd_24826148_45296490() {
        assert_eq!(gcd(Int::from_str("24826148", 10), Int::from_str("45296490", 10), Int::zero(), Int::one()).0.to_str(10), "526")
    }
    #[test]
    fn gcd_12_0() {
        assert_eq!(gcd(Int::from_str("12", 10), Int::from_str("0", 10), Int::zero(), Int::one()).0.to_str(10), "12")
    }
    #[test]
    fn gcd_0_0() {
        assert_eq!(gcd(Int::from_str("0", 10), Int::from_str("0", 10), Int::zero(), Int::one()).0.to_str(10), "0")
    }
    #[test]
    fn gcd_0_9() {
        assert_eq!(gcd(Int::from_str("0", 10), Int::from_str("9", 10), Int::zero(), Int::one()).0.to_str(10), "9")
    }

    #[test]
    fn mod_inv_1_2() {
        assert_eq!(main(&Int::from_str("1", 10), &Int::from_str("2", 10)).to_str(10), "1")
    }
    #[test]
    #[should_panic]
    fn mod_inv_3_6() {
        let _ans = main(&Int::from_str("3", 10), &Int::from_str("6", 10));
    }
    
    #[test]
    fn mod_inv_7_87() {
        assert_eq!(main(&Int::from_str("7", 10), &Int::from_str("87", 10)).to_str(10), "25")
    }
    
    #[test]
    fn mod_inv_25_87() {
        assert_eq!(main(&Int::from_str("25", 10), &Int::from_str("87", 10)).to_str(10), "7")
    }
    
    #[test]
    fn mod_inv_2_91() {
        assert_eq!(main(&Int::from_str("2", 10), &Int::from_str("91", 10)).to_str(10), "46")
    }

    #[test]
    fn mod_inv_19_1212393831() {
        assert_eq!(main(&Int::from_str("19", 10), &Int::from_str("1212393831", 10)).to_str(10), "701912218")
    }

    #[test]
    fn mod_inv_31_73714876143() {
        assert_eq!(main(&Int::from_str("31", 10), &Int::from_str("73714876143", 10)).to_str(10), "45180085378")
    }
    #[test]
    #[should_panic]
    fn mod_inv_3_73714876143() {
        let _ans = main(&Int::from_str("3", 10), &Int::from_str("73714876143", 10));
    }

}