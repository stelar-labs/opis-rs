use crate::Int;

// use crate::int::modulo;

pub fn main(a: &Int, m: &Int) -> Int {

    let (g, mut x) = gcd(a.clone(), m.clone(), Int::zero(), Int::one()); 
    
    if g != Int::one() {
        panic!("No solution!")
    }
    
    else {

        while x < Int::zero() {
            x += m.clone();
        }

        x

    }

}

fn gcd(a: Int, m: Int, x: Int, y: Int) -> (Int, Int) {

    if a == Int::zero() {
        (m, x)
    }
    
    else {

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
        assert_eq!(
            gcd(Int::from("42"), Int::from("56"), Int::zero(), Int::one()).0.to(10),
            "14"
        )
    }

    #[test]
    fn gcd_461952_116298() {
        assert_eq!(
            gcd(Int::from("461952"), Int::from("116298"), Int::zero(), Int::one()).0.to(10),
            "18"
        )
    }

    #[test]
    fn gcd_7966496_314080416() {
        assert_eq!(
            gcd(Int::from("7966496"), Int::from("314080416"), Int::zero(), Int::one()).0.to(10),
            "32"
        )
    }

    #[test]
    fn gcd_24826148_45296490() {
        assert_eq!(
            gcd(Int::from("24826148"), Int::from("45296490"), Int::zero(), Int::one()).0.to(10),
            "526"
        )
    }

    #[test]
    fn gcd_12_0() {
        assert_eq!(
            gcd(Int::from("12"), Int::from("0"), Int::zero(), Int::one()).0.to(10),
            "12"
        )
    }

    #[test]
    fn gcd_0_0() {
        assert_eq!(
            gcd(Int::from("0"), Int::from("0"), Int::zero(), Int::one()).0.to(10),
            "0"
        )
    }

    #[test]
    fn gcd_0_9() {
        assert_eq!(
            gcd(Int::from("0"), Int::from("9"), Int::zero(), Int::one()).0.to(10),
            "9"
        )
    }

    #[test]
    fn mod_inv_1_2() {
        assert_eq!(
            main(&Int::from("1"), &Int::from("2")).to(10),
            "1"
        )
    }

    #[test]
    #[should_panic]
    fn mod_inv_3_6() {
        let _ans = main(&Int::from("3"), &Int::from("6"));
    }
    
    #[test]
    fn mod_inv_7_87() {
        assert_eq!(
            main(&Int::from("7"), &Int::from("87")).to(10),
            "25"
        )
    }
    
    #[test]
    fn mod_inv_25_87() {
        assert_eq!(
            main(&Int::from("25"), &Int::from("87")).to(10),
            "7"
        )
    }
    
    #[test]
    fn mod_inv_2_91() {
        assert_eq!(
            main(&Int::from("2"), &Int::from("91")).to(10),
            "46"
        )
    }

    #[test]
    fn mod_inv_19_1212393831() {
        assert_eq!(
            main(&Int::from("19"), &Int::from("1212393831")).to(10),
            "701912218"
        )
    }

    #[test]
    fn mod_inv_31_73714876143() {
        assert_eq!(
            main(&Int::from("31"), &Int::from("73714876143")).to(10),
            "45180085378"
        )
    }

    #[test]
    #[should_panic]
    fn mod_inv_3_73714876143() {
        let _ans = main(&Int::from("3"), &Int::from("73714876143"));
    }

}