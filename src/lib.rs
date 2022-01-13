use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, Rem, Not, BitAnd, BitOr, BitXor};

mod base10;

#[derive(Clone, Debug)]
pub struct Int {
    pub bits: Vec<u8>,
    pub sign: bool
}

impl Int {

    pub fn zero() -> Self { Int{bits: vec![0], sign: false} }

    pub fn one() -> Self { Int{bits: vec![1], sign: false} }

    pub fn from_str(s: &str, r: u8) -> Self {

        match r {

            2 => {

                let mut split_str: Vec<_> = s.split("").collect();
    
                split_str.retain(|&x| x != "");

                let bits = split_str
                    .iter()
                    .skip(2)
                    .map(|&x| u8::from_str_radix(x, 2).unwrap())
                    .collect();

                Int {bits: bits, sign: false}

            },

            10 => {

                let b = base10::from(s).unwrap();

                Int {bits: b, sign: false}

            },

            16 => {

                let mut res: Vec<u8> = vec![];

                let mut hex_str_vec: Vec<_> = s.split("").collect();
                
                hex_str_vec.retain(|&x| x != "");

                hex_str_vec
                    .iter()
                    .skip(2)
                    .for_each(|x| {
                        
                        let byte = u8::from_str_radix(&format!("{}", x), 16).unwrap();

                        let binary_str = format!("{:04b}", byte);

                        let mut binary_str_vec: Vec<_> = binary_str.split("").collect();
                
                        binary_str_vec.retain(|&x| x != "");

                        binary_str_vec
                            .iter()
                            .for_each(|y| {
                                
                                let bit = u8::from_str_radix(&y, 2).unwrap();

                                res.push(bit)

                            });

                    });

                while res.len() > 1 && res[0] == 0 {
                    res.remove(0);
                }

                Int {bits: res, sign: false}

            },

            _ => panic!("Unsupported radix!")

        }

    }

    pub fn to_str(self, r: u8) -> String {
        
        match r {
            
            2 => {

                let mut res: String = String::with_capacity(self.bits.len() + 2);

                res.push_str("b'");
    
                for bit in self.bits {
                    res.push_str(&bit.to_string())
                };

                res

            },

            10 => base10::to(self.bits),

            16 => {

                let mut res = String::from("0x");

                let bytes: Vec<u8> = self.to_bytes();

                bytes
                    .iter()
                    .for_each(|x| {

                        let hex_str = format!("{:02X}", x);

                        res.push_str(&hex_str)

                    });
                    
                res

            },

            _ => panic!("Unsupported radix!")

        }

    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Int {

        let mut res = Vec::new();

        let bin_str: String = bytes
            .iter()
            .fold(
                String::new(), |acc, x| {
                    let mut new = acc;
                    new.push_str(&format!("{:08b}", x));
                    new
                }
            );

        for i in bin_str.chars() {
            res.push(u8::from_str_radix(&i.to_string(), 2).unwrap())
        }

        while res.len() > 1 && res[0] == 0 {
            res.remove(0);
        }

        Int {bits: res, sign: false}

    }

    pub fn to_bytes(mut self) -> Vec<u8> {

        let mut res: Vec<u8> = Vec::new();

        while !self.bits.is_empty() {

            let mut byte_bits: Vec<u8> = Vec::new();

            while byte_bits.len() != 8 {

                match self.bits.pop() {

                    Some(r) => byte_bits.push(r),
                    
                    None => break

                }

            }

            byte_bits.reverse();

            let byte_str = byte_bits
                .iter()
                .fold(
                    String::new(), |acc, x| {
                        let mut new = acc;
                        new.push_str(&format!("{}", x));
                        new
                    }
                );

            res.push(u8::from_str_radix(&byte_str, 2).unwrap())

        }

        res.reverse();

        res


    }

}

impl Add for Int {

    type Output = Self;

    fn add(self, b: Self) -> Self {
        
        if !self.sign && !b.sign {
            Self {bits: adder(&self.bits, &b.bits), sign: false}
        }
        
        else if self.sign && b.sign {
            Self {bits: adder(&self.bits, &b.bits), sign: true}
        }
            
        else if self.sign {
            b - Self {bits: self.bits, sign: true}
        }
        
        else {
            self - Self {bits: b.bits, sign: true}
        }

    }

}

impl Add for &Int {

    type Output = Int;

    fn add(self, b: Self) -> Int {
        self.clone() + b.clone()
    }

}

impl AddAssign for Int {
    fn add_assign(&mut self, b: Self) {
        *self = self.clone() + b;
    }
}

impl Sub for Int {

    type Output = Self;

    fn sub(self, b: Self) -> Self {
        
        if !self.sign && !b.sign {

            if self > b {
                Self {bits: subtractor(&self.bits, &b.bits), sign: false}
            }
            
            else {
                Self {bits: subtractor(&b.bits, &self.bits), sign: true}
            }

        }

        else if self.sign && b.sign {
            self + Self {bits: b.bits, sign: false}
        }
        
        else if self.sign {
            Self {bits: adder(&self.bits, &b.bits), sign: true}
        }
        
        else {
            Self {bits: adder(&self.bits, &b.bits), sign: false}
        }

    }
}

impl Sub for &Int {

    type Output = Int;

    fn sub(self, b: Self) -> Int {
        self.clone() - b.clone()
    }

}

impl SubAssign for Int {
    fn sub_assign(&mut self, b: Self) {
        *self = self.clone() - b;
    }
}

impl Mul for Int {

    type Output = Self;

    fn mul(self, b: Self) -> Self {

        let mut res = Int::zero();

        if self.bits != vec![0] && b.bits != vec![0] {

            res = self.clone();

            b.bits
                .iter()
                .skip(1)
                .for_each(|&x| {

                    res.bits = adder(&res.bits, &res.bits);
                    
                    if x == 1 {
                        res.bits = adder(&res.bits, &self.bits);
                    }
                
                });

            match(self.sign, b.sign) {
                (false, true) => res.sign = true,
                (true, false) => res.sign = true,
                _ => ()
            }

        }

        res

    }

}

impl Mul for &Int {

    type Output = Int;

    fn mul(self, b: Self) -> Int {
        self.clone() * b.clone()
    }

}

impl Div for Int {

    type Output = Self;

    fn div(self, b: Self) -> Self {

        let mut res = Int::zero();

        if self.bits == vec![0] {
            res
        }
        
        else if b.bits == vec![0] {
            panic!("a/0 is undefined!")
        }
        
        else {

            let (q, _) = divisor(&self.bits, &b.bits);
            
            res = Int{bits: q, sign: false};

            match(self.sign, b.sign) {
                (false, true) => res.sign = true,
                (true, false) => res.sign = true,
                _ => ()
            }

            res

        }

    }

}

impl Div for &Int {

    type Output = Int;

    fn div(self, b: Self) -> Int {
        self.clone() / b.clone()
    }

}

impl Rem for Int {

    type Output = Self;

    fn rem(self, b: Self) -> Self {

        if self.bits == vec![0] {
            Int::zero()
        }
        
        else if b.bits == vec![0] {
            panic!("a/0 is undefined!")
        }
        
        else {

            let (_, r) = divisor(&self.bits, &b.bits);

            let mut res = Int{bits: r, sign: false};

            if self.sign {
                res.sign = true;
            }

            res

        }

    }

}

impl Rem for &Int {

    type Output = Int;

    fn rem(self, b: Self) -> Int {
        self.clone() % b.clone()
    }

}

impl Not for Int {

    type Output = Self;

    fn not(mut self) -> Self::Output {
        
        self.bits = self.bits
            .iter()
            .map(|&x| {
                match x {
                    1 => 0,
                    _ => 1
                }
            })
            .collect();

        self

    }

}

impl Not for &Int {

    type Output = Int;

    fn not(self) -> Int {
        !self.clone()
    }

}

impl BitAnd for Int {

    type Output = Self;
    
    fn bitand(mut self, mut b: Self) -> Self::Output {

        let mut res: Vec<u8> = Vec::new();

        while !self.bits.is_empty() || !b.bits.is_empty() {

            let a_bit = match self.bits.pop() {
                Some(r) => r,
                None => 0
            };
    
            let b_bit = match b.bits.pop() {
                Some(r) => r,
                None => 0
            };

            let and_bit = match (a_bit, b_bit) {
                (1, 1) => 1,
                _ => 0
            };

            res.push(and_bit)

        }

        res.reverse();

        Int {bits: res, sign: false}

    }
}

impl BitAnd for &Int {

    type Output = Int;

    fn bitand(self, b: Self) -> Int {
        self.clone() & b.clone()
    }

}

impl BitOr for Int {

    type Output = Self;

    fn bitor(mut self, mut b: Self) -> Self::Output {

        let mut res: Vec<u8> = Vec::new();

        while !self.bits.is_empty() || !b.bits.is_empty() {

            let a_bit = match self.bits.pop() {
                Some(r) => r,
                None => 0
            };
    
            let b_bit = match b.bits.pop() {
                Some(r) => r,
                None => 0
            };

            let or_bit = match (a_bit, b_bit) {
                (0, 0) => 0,
                _ => 1
            };

            res.push(or_bit)

        }

        res.reverse();

        Int {bits: res, sign: false}

    }
}

impl BitOr for &Int {

    type Output = Int;

    fn bitor(self, b: Self) -> Int {
        self.clone() | b.clone()
    }

}

impl BitXor for Int {

    type Output = Self;

    fn bitxor(mut self, mut b: Self) -> Self::Output {

        let mut res: Vec<u8> = Vec::new();

        while !self.bits.is_empty() || !b.bits.is_empty() {

            let a_bit = match self.bits.pop() {
                Some(r) => r,
                None => 0
            };
    
            let b_bit = match b.bits.pop() {
                Some(r) => r,
                None => 0
            };

            let xor_bit = match (a_bit, b_bit) {
                (0, 1) => 1,
                (1, 0) => 1,
                _ => 0
            };

            res.push(xor_bit)

        }

        res.reverse();

        Int {bits: res, sign: false}

    }
}

impl BitXor for &Int {

    type Output = Int;

    fn bitxor(self, b: Self) -> Int {
        self.clone() ^ b.clone()
    }

}

impl Ord for Int {

    fn cmp(&self, b: &Self) -> Ordering {
        
        if !self.sign && b.sign {
            Ordering::Greater
        }
        
        else if self.sign && !b.sign {
            Ordering::Less
        }
        
        else if !self.sign && !b.sign {
            comparator(&self.bits, &b.bits)
        }
        
        else {
    
            match comparator(&self.bits, &b.bits) {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal
            }
    
        }

    }

}

impl PartialOrd for Int {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}

impl PartialEq for Int {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits && self.sign == other.sign
    }
}

impl Eq for Int {}

fn adder(a_bits: &Vec<u8>, b_bits: &Vec<u8>) -> Vec<u8> {

    let mut a = a_bits.clone();

    let mut b = b_bits.clone();
    
    let mut res: Vec<u8> = Vec::new();

    let mut carry: u8 = 0;

    while !a.is_empty() || !b.is_empty() {

        let a_bit = match a.pop() {
            Some(r) => r,
            None => 0
        };

        let b_bit = match b.pop() {
            Some(r) => r,
            None => 0
        };

        let addition = carry + a_bit + b_bit;

        match addition {
            3 => { res.push(1); carry = 1 },
            2 => { res.push(0); carry = 1 },
            1 => { res.push(1); carry = 0 },
            _ => { res.push(0); carry = 0 }
        }

    }

    if carry != 0 {
        res.push(1);
    }

    res.reverse();

    res

}

fn subtractor(a_bits: &Vec<u8>, b_bits: &Vec<u8>) -> Vec<u8> {

    let mut a = a_bits.clone();
    let mut b = b_bits.clone();
    
    let mut res: Vec<u8> = Vec::new();

    while !a.is_empty() || !b.is_empty() {

        let a_bit = match a.pop() {
            Some(r) => r,
            None => 0
        };

        let b_bit = match b.pop() {
            Some(r) => r,
            None => 0
        };

        let bits: (u8, u8) = (a_bit, b_bit);

        match bits {
            (0,0) => res.push(0),
            (1,1) => res.push(0),
            (1,0) => res.push(1),
            _ => {

                let mut borrowed: bool = false;

                let mut borrow_index = a.len() - 1;

                while !borrowed {

                    if a[borrow_index] == 1 {
                        
                        a[borrow_index] = 0;
                        
                        borrowed = true;
                    
                    }
                    
                    else {
                        
                        a[borrow_index] = 1;
                        
                        borrow_index -= 1;
                    
                    }
    
                }
                
                res.push(1);

            }
        }

    }

    res.reverse();

    while res.len() > 1 && res[0] == 0 {
        res.remove(0);
    }

    res

}

fn divisor(a: &Vec<u8>, b: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {

    let mut q: Vec<u8> = vec![0];

    let mut r: Vec<u8> = vec![0];

    a.iter()
        .for_each(|&x| {

            r.push(x);

            while r.len() > 1 && r[0] == 0 {
                r.remove(0);
            };

            if comparator(&r, b) == Ordering::Greater {

                q.push(1);
                
                r = subtractor(&r, b);
                
            }
            
            else if comparator(&r, b) == Ordering::Equal {

                q.push(1);

                r = vec![0]

            }
            
            else {
                q.push(0)
            };

        });

    while q.len() > 1 && q[0] == 0 {
        q.remove(0);
    };

    while r.len() > 1 && r[0] == 0 {
        r.remove(0);
    };

    (q, r)

}

fn comparator(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {

    let mut a_bits = a.clone();
    
    let mut b_bits = b.clone();

    while a_bits.len() > 1 && a_bits[0] == 0 {
        a_bits.remove(0);
    };

    while b_bits.len() > 1 && b_bits[0] == 0 {
        b_bits.remove(0);
    };

    let a_len = a_bits.len();
    
    let b_len = b_bits.len();

    if a_len > b_len {
        Ordering::Greater
    }
    
    else if a_len < b_len {
        Ordering::Less
    }
    
    else {

        if a_bits == b_bits {
            Ordering::Equal
        }
        
        else {

            while a_bits[0] == b_bits[0] { 
                
                a_bits.remove(0);
                
                b_bits.remove(0);

            }

            if a_bits[0] == 1 {
                Ordering::Greater
            }
            
            else {
                Ordering::Less
            }

        }

    }

}

pub fn pow(a: &Int, e:&Int) -> Int {

    if a.bits == vec![0] {
        Int::zero()
    }
    
    else if e.bits == vec![0] {
        Int::one()
    }
    
    else {

        let mut res: Int = a.clone();

        e.bits
            .iter()
            .skip(1)
            .for_each(|&x| {

                res = (&res * &res).clone();

                if x == 1 {
                    res = (&res * a).clone()
                }
                
            });

        res

    }

}

pub fn modulo(a: &Int, b: &Int) -> Int {

    if a.bits == vec![0] {
        Int::zero()
    }
    
    else if b.bits == vec![0] {
        panic!("a/0 is undefined!")
    }
    
    else {

        let r = a % b;

        if r.sign { &r + b }
        
        else { r }

    }   

} 
  
pub fn mod_inv(a: &Int, m: &Int) -> Int {

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
