# Opis

Opis is a library for rational number and matrix arithmetic.

## Author

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

- Arbitrary Precision Integer Representation and Arithmetic
- Fractions Arithmetic
- Matrix Arithmetic & Linear Regression

## Usage

### Import

```text
use opis::{ Bit, Integer, Fraction, Matrix };
```

### Bit

Add `a + b`

And `a & b`

Eq `a == a`

Not `!a`

Or `a | b`

Xor `a ^ b`

### Integer

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

### Fraction

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

### Matrix

#### Addition
`fn add(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>>`

#### Cofactors
```
A = [ 3 -1 -2]   C = [ 3  1 -2]
    [ 3  1 -1]        [-3  1  1]
    [ 3  4  2]        [ 3 -4  2]
```
`fn cofactors(&self, neg_one: T) -> Result<Matrix<T>, Box<dyn Error>>`

#### Determinant
`fn determinant(&self, neg_one: T) -> Result<T, Box<dyn std::error::Error>>`

#### Dimensions
`fn dimensions(&self) -> Result<(usize, usize), Box<dyn Error>>`

#### Equality
`fn eq(&self, b: &Self) -> bool`

#### Identity
`fn identity(size: usize, zero: T, one: T) -> Matrix<T>`

#### inverse
`fn inverse(&self, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>>`

#### Linear Regression
`fn linear_regression(&self, variables: &Matrix<T>, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>>`

#### Minors
`fn minors(&self, neg_one: T) -> Result<Matrix<T>, Box<dyn Error>>`

#### Multiplication
`fn mul(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>>`
`fn mul(self, b: T) -> Matrix<T>`

#### Subtraction
`fn sub(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>>`

#### Trace
```
A = [-1  2  7  0]   Tr(A) = (-1 + 5 + 7 + 0) = 11
    [ 3  5 -8  4]
    [ 1  2  7 -3]
    [ 4 -2  1  0]
```
`fn trace(&self) -> Result<T, Box<dyn Error>>`

#### Transpose
```
A = [2 0]   A' = [2 1 4]
    [1 1]        [0 1 3]
    [4 3]
```
`fn transpose(&self) -> Result<Matrix<T>, Box<dyn Error>>`

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
