# Opis

Opis is a library for Number and Matrix Arithmetic.

## Author

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

- Arbitrary Precision Number Representation and Arithmetic
- Matrix Arithmetic

## Usage

### Installation

- From [Crates](https://crates.io/) by running `cargo add opis`
- From Crates by adding `opis = "6.0.0"` to `Cargo.toml` under `[dependencies]`

### Bit

- Add `a + b -> cd`
- And `a & b -> c`
- Eq `a == a -> bool`
- Not `!a -> b`
- Or `a | b -> c`
- Xor `a ^ b -> c`

### Integer

- Addition `a + b -> c, a += b`
- And `a & b -> c`
- Bytes `Integer::from(&bytes), a.into()`
- Comparision `a < b, a <= b, a > b, a >= b`
- Division `a / b -> c/error`
- Equality `a == b -> bool`
- Extended Euclidean Algorithm `a.ext_gcd(&b) -> c,d,e`
- Extended Bits `a.to_ext_bits(256) -> Vec<Bits>`
- Extended Bytes `a.to_ext_bytes(32) -> Vec<u8>`
- Inversion `a.inversion() -> b`
- Modulo `a.modulo(&m) -> b/error`
- Modular Exponentiation `a.mod_pow(&e, &m) -> b/error`
- Multiply `a * b -> c, a *= b`
- Not `!a -> b`
- Or `a | b -> c`
- Exponentiation `a.pow(e) -> b/error`
- Remainder: `a % b -> c/error`
- Shifts `a << 1 -> b, a <<= 1, a >> 1 -> b, a >>= 1`
- String Conversion `Integer::try_from(&s) -> a/error`
-- Binary (b'11, b'00, b'01)
-- Decimal (-1, 0, 1)
-- Hexadecimal (0xFF, 0x00, 0x01)
-- Floating Literal (3e3)
- Subtraction `a - b -> c, a -= b`
- Type Conversion `&[Bits], &[u8], u8, u16, u32, u64, u128, usize`

<!-- Base2 `Integer::from_bin("1010101"), a.to_bin()` -->
<!-- Base10 `Integer::from_dec("674755"), a.to_dec()` -->
<!-- Base16 `Integer::from_hex("00ABC012"), a.to_hex()` -->
<!-- Linear Feedback Shift Register `a.lfsr(1)?` -->

### Fraction

- Addition `(a,b) + (c,d) = (ad + bc, bd)`
- Subtraction `(a,b) - (c,d) = (ad - bc, bd)`
- Multiplication `(a,b) * (c,d) = (ac, bd)`
- Division `(a,b) / (c,d) = (ad, bc)/error`
- Comparison & Equality
- Additive Inverse / Opposite `-(a,b).opposite = (-a,b)`
- Multiplicative Inverse / Reciprocal `(a,b).reciprocal() = (b,a)/error`
- Reduction `a.reduction() -> b`
- String Conversion: Binary, Decimal, Hexadecimal, Floating Literal
- Type Conversion: Integer, u8, u16, u32, u64, u128, usize

### Matrix

- Addition `A + B -> C/error`
- Cofactors `A.cofactors() -> C/error`
- Determinant `A.det() -> d/error`
- Size `A.size() -> (rows, columns)/error`
- Equality `A == B -> bool`
- Identity `Matrix::identity(size) -> I`
- Inverse `A.inverse() -> A^-1/error`
- Linear Regression `A.linear_regression(V) -> B/error`
- Minors `A.minors() -> M/error`
- Multiplication `A * B -> C/error, c * A -> B`
- Subtraction `A - B -> C/error`
- Trace `A.trace() -> t/error`
- Transpose `A.transpose() -> A^T/error`

## License

MIT License

Copyright Stelar Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
