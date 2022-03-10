## Rust Opis
Opis is an arithmetic library for integer numbers written in Rust.

### Usage

In your `Cargo.toml`:

```

[dependencies]
opis = "3.0.2"

```

In your Rust file:

```

use opis::{ Bit, Int, pow, modulo };

```

### Features
- Representation of integers of any sign & magnitude.
- The Magnitude is stored in big endian order.
- Std Ops Int support for:-
    - Add
    - AddAssign
    - Sub
    - SubAssign
    - Mul
    - Div
    - Rem
    - Not
    - BitAnd
    - BitOr
    - BitXor
    - Eq
    - Ord
- Arithetic functions included are:-
    - pow - Exponentiation
    - modulo
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

#### Conversion Functions

`From Str`
```

let binary_integer: Int = Int::from_binary("b'1010101");

let decimal_integer: Int = Int::from_decimal("674755");

let hex_integer: Int = Int::from_hex("0x00ABC012");

```

`To Str`
```

let binary_str: String = integer_1.to_binary();

let decimal_str: String = integer_2.to_decimal();

let hex_str: String = integer_3.to_hex();
```

`From Bytes`
```

let integer: Int = Int::from_bytes(&bytes);

```

`To Bytes`
```

let bytes: Vec<u8> = integer.to_bytes();

let ext_bytes: Vec<u8> = integer.to_ext_bytes(32);

```



### Future
- Benchmarking Performance.

### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-03-01
