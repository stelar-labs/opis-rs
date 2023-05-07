# Opis

Opis is a library for rational number and matrix arithmetic.

## Author

Roy R. O. Okello

[Email](mailto:royokello@protonmail.com)

[Github](https://github.com/royokello)

[Twitter](https://twitter.com/RealOkello)

## Features

- Arbitrary Precision Integer Representation and Arithmetic
- Fractions

## Usage

### Cargo.toml

```text
[dependencies]
opis = "5.4.0"
```

### Module.rs

```text
use opis::{ Bit, Integer, Fraction, Matrix };
```

### API

#### Bit

Add `a + b`

And `a & b`

Eq `a == a`

Not `!a`

Or `a | b`

Xor `a ^ b`

#### Integer

Addition `a + b, a += b`

And `a & b`

Base2 `Integer::from_bin("1010101"), a.to_bin()`

Base10 `Integer::from_dec("674755"), a.to_dec()`

Base16 `Integer::from_hex("00ABC012"), a.to_hex()`

Bytes `Integer::from(&bytes), a.into()`

Comparision `a < b, a <= b, a > b, a >= b`

Division `a / b?`

Equality `a == b`

Extended Euclidean Algorithm `a.ext_gcd(&b)`

Extended Bits `a.to_ext_bits(256)`

Extended Bytes `a.to_ext_bytes(32)`

Linear Feedback Shift Register `a.lfsr(1)?`

Modulo: `a.modulo(&m)`

Modular Exponentiation: `base.mod_pow(&exponent, &modulus)`

Multiply `a * b, a *= b`

Negate `a.negate()`

Not `!a`

Or `a | b`

Exponentiation `a.pow(e)`

Remainder: `a % b?`

Shifts `a << 1, a <<= 1, a >> 1, a >>= 1`

Subtraction `a - b, a -= b`

Type Conversion `2_u8.into()`

#### Fraction

Addition `a + b, a += b`

Comparison `a < b, a <= b, a > b, a >= b`

Division `a / b?`

Equality `a == b`

Multiplication `a * b, a *= b`

Reciprocal `a.reciprocal()`

Reduce `a.reduce()`

String Conversion `Fraction::try_from("1/2")`

Subtraction `a - b, a -= b`

Type Conversion `2_u8.into()`

#### Matrix

Addition `a + b`

Cofactors `a.cofactors(neg_one, one)`

Determinant `a.determinant()`

Dimensions `a.dimensions()`

Equality `a == b`

Identity `Matrix::identity(size, zero, one)`

Inverse `a.inverse(zero, one)`

Minors `a.minors()`

Multiplication `a * b`

Subtraction `a - b`

Trace `a.trace()`

Transpose `a.transpose()`

## License

## Disclaimer

12023-05-07
