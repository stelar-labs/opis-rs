# Opis

Opis is an arithmetic library for integer numbers.

## Usage

In your `Cargo.toml`:

```text
[dependencies]
opis = "3.1.0"
```

In your `module.rs`:

```text
use opis::{ Bit, Int, pow, modulo };
```

## Features

- Representation of integers of any sign & magnitude.
- The Magnitude is stored in big endian order.
- Std Ops Int support for Add, AddAssign, Sub, SubAssign, Mul, Div, Rem, Not, BitAnd, BitOr, BitXor, Eq and Ord.
- Arithetic functions included are pow - exponentiation and modulo.
- Conversion functions support for radix 2, 10 & 16 formatted strings.
- Linear-feedback shift register.

## API

### Operators

`Add`

```text
let sum: Int = a + b;
```

`AddAssign`

```text
let total: Int = a += b;
```

`Sub`

```text
let sum: Int = a - b;
```

`SubAssign`

```text
let reduced: Int = a -= b;
```

`Mul`

```text
let product: Int = a * b;
```

`Div`

```text
let quotient: Int = a / b;
```

`Rem`

```text
let rem: Int = a % b;
```

`Not`

```text
let not_a: Int = !a;
```

`BitAnd`

```text
let a_and_b: Int = a & b;
```

`BitOr`

```text
let a_or_b: Int = a | b;
```

`BitXor`

```text
let a_xor_b: Int = a ^ b;
```

`Eq`

```text
if a == b {
    println!("equal!")
}

if a != b {
    println!("not equal!")
}
```

`Ord`

```text
if a > b {
    println!("a is greater!")
}

if a < b {
    println!("a is less!")
}
```

### Arithetic Functions

`Exponentiation`

```text
use opis::pow;

let e: Int = pow(&a, &b);
```

`Modulo`

```text
use opis::modulo;

let m: Int = modulo(&a, &b);
```

### Conversion Functions

`From Str`

```text
let binary_integer: Int = Int::from_binary("b'1010101");

let decimal_integer: Int = Int::from_decimal("674755");

let hex_integer: Int = Int::from_hex("0x00ABC012");
```

`To Str`

```text
let binary_str: String = integer_1.to_binary();

let decimal_str: String = integer_2.to_decimal();

let hex_str: String = integer_3.to_hex();
```

`From Bytes`

```text
let integer: Int = Int::from_bytes(&bytes);
```

`To Bytes`

```text
let bytes: Vec<u8> = integer.to_bytes();

let ext_bytes: Vec<u8> = integer.to_ext_bytes(32);
```

#### Linear-feedback shift register

```text
let int_lfsr: Int = integer.lfsr();
```

## Improvements

- Performance
- Two's Complement Format
- Extended Euclidean Algorithm
- Modular Arithmetic
- Fibonacci Linear-Feedback Shift Register

## Contribution

Pull requests, bug reports and any kind of suggestion are welcome.

2022-04-30
