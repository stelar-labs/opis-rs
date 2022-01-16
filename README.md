## Rust Opis
Opis is an arithmetic library for integer numbers written in Rust.

### Usage

In your `Cargo.toml`:

```
[dependencies]
opis = "2.1.0"
```

In your Rust file:

```
use opis::Int;
```

### Features
- Representation of integers of any sign & magnitude.
- Bits are stored in big endian order.
- Operator support for:-
    - Add - addition
    - AddAssign - add and assign
    - Sub - subtraction
    - SubAssign - subtract and assign
    - Mul - multiplication
    - Div - division
    - Rem - remainder
    - Not
    - BitAnd
    - BitOr
    - BitXor
    - Eq - equality comparison
    - Ord - ordering comparison
- Arithetic functions included are:-
    - pow - exponentiation
    - modulo
    - mod_inv - modular multiplicative inverse
- Conversion functions support for radix 2, 10 & 16 formatted strings.

### API

#### Operators

`Add`
```
let sum: Int = a + b;
```

`AddAssign`
```
let total: Int = a += b;
```

`Sub`
```
let sum: Int = a - b;
```

`SubAssign`
```
let reduced: Int = a -= b;
```

`Mul`
```
let product: Int = a * b;
```

`Div`
```
let quotient: Int = a / b;
```

`Rem`
```
let rem: Int = a % b;
```

`Not`
```
let not_a: Int = !a;
```

`BitAnd`
```
let a_and_b: Int = a & b;
```

`BitOr`
```
let a_or_b: Int = a | b;
```

`BitXor`
```
let a_xor_b: Int = a ^ b;
```

`Eq`
```
if a == b {
    println!("equal!")
}

if a != b {
    println!("not equal!")
}
```

`Ord`
```
if a > b {
    println!("a is greater!")
}

if a < b {
    println!("a is less!")
}
```

#### Arithetic Functions

`Exponentiation`
```
use opis::pow;

let e: Int = pow(&a, &b);
```

`Modulo`
```
use opis::modulo;

let m: Int = modulo(&a, &b);
```

`Modular multiplicative inverse`
```
use opis::mod_inv;

let i: Int = mod_inv(&a, &b);
```

#### Conversion Functions

`From Str`
```
let binary_integer: Int = Int::from_str("b'1010101", 2);

let decimal_integer: Int = Int::from_str("674755", 10);

let hex_integer: Int = Int::from_str("0xABC012", 16);
```

`To Str`
```
let binary_str: String = integer_1.to_str(2);

let decimal_str: String = integer_2.to_str(10);

let hex_str: String = integer_3.to_str(16);
```

`From Bytes`
```
let integer: Int = Int::from_bytes(&bytes);
```

`To Bytes`
```
let bytes: Vec<u8> = integer.to_bytes();
```



### Future
- Benchmarking Performance
- Twos Complements support

### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-01-13
